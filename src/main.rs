use std::sync::Arc;

use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::{
    CentralPanel, Context, FontData, FontDefinitions, FontFamily, SidePanel, TopBottomPanel,
    ViewportCommand,
};

mod canvas;
mod center_widget;
mod clipboard;
mod eraser;
mod export;
mod footer;
mod header;
mod history;
mod palette;
mod screenshot;
mod selection;
mod state;

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1680.0, 1050.0]),
        ..Default::default()
    };

    run_native(
        "Snap",
        native_options,
        Box::new(|cc| Ok(Box::new(Snap::new().init(cc)))),
    )
}

pub trait Widget {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, state: &mut state::AppState);
}

pub trait View {
    /// Render into a panel, with access to shared application state.
    fn render(&mut self, ui: &mut egui::Ui, state: &mut state::AppState);
}

struct Snap {
    dark_mode: bool,
    header: header::Header,
    canvas: canvas::Canvas,
    footer: footer::Footer,
    state: state::AppState,
}

impl Snap {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            footer: footer::Footer::new(),
            canvas: canvas::Canvas::new(),
            header: header::Header::new(),
            state: state::AppState::default(),
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        self.configure_fonts(&cc.egui_ctx);
        self.dark_mode = self.detect_os_theme();
        self.apply_theme(&cc.egui_ctx);

        self
    }

    fn detect_os_theme(&self) -> bool {
        match dark_light::detect() {
            Ok(dark_light::Mode::Light) => false,
            // Default to dark for Dark, Unspecified, or detection errors
            _ => true,
        }
    }

    fn apply_theme(&self, ctx: &Context) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "MesloLGM".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/MesloLGM.ttf"
            ))),
        );

        // Set first proportional font
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "MesloLGM".to_owned());

        // Set first monospaced font
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "MesloLGM".to_owned());

        ctx.set_fonts(fonts);
    }
}

/// Number of frames to wait after minimising before capturing, giving the
/// window manager time to actually hide the window.
const MINIMISE_WAIT_FRAMES: u32 = 5;

impl App for Snap {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Drive the screenshot capture state machine before rendering UI.
        self.tick_capture(ctx);

        // Ctrl+C: copy canvas region to clipboard
        if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.command) {
            self.copy_canvas_to_clipboard();
        }

        // Ctrl+V: paste image from clipboard
        if ctx.input(|i| i.key_pressed(egui::Key::V) && i.modifiers.command) {
            self.paste_image_from_clipboard(ctx);
        }

        // Handle undo/redo keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && !i.modifiers.shift) {
            self.state.history.undo(&mut self.state.objects);
        }
        if ctx.input(|i| {
            i.key_pressed(egui::Key::Y) && i.modifiers.ctrl
                || i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && i.modifiers.shift
        }) {
            self.state.history.redo(&mut self.state.objects);
        }

        TopBottomPanel::top("top_panel")
            .exact_height(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    self.header.render(ui, &mut self.state);
                })
            });

        if self.header.take_theme_toggled() {
            self.dark_mode = !self.dark_mode;
            self.apply_theme(ctx);
        }

        if self.header.take_undo_clicked() {
            self.state.history.undo(&mut self.state.objects);
        }
        if self.header.take_redo_clicked() {
            self.state.history.redo(&mut self.state.objects);
        }

        if std::mem::take(&mut self.state.export_requested) {
            let bg = if self.dark_mode {
                egui::Color32::from_gray(27) // egui dark background
            } else {
                egui::Color32::from_gray(248) // egui light background
            };
            // Use a sensible default canvas size for the export
            let viewport = ctx.input(|i| i.viewport_rect());
            let w = viewport.width().max(1.0) as u32;
            let h = viewport.height().max(1.0) as u32;
            if let Err(e) = export::export_with_dialog(&self.state.objects, w, h, bg) {
                eprintln!("Export failed: {e}");
            }
        }

        TopBottomPanel::bottom("bottom_panel")
            .exact_height(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    self.footer.render(ui, &mut self.state);
                })
            });

        SidePanel::left("left_panel")
            .exact_width(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| ui.label("left"));

        CentralPanel::default().show(ctx, |ui| self.canvas.render(ui, &mut self.state));
    }
}

impl Snap {
    /// Advances the screenshot capture state machine by one frame.
    fn tick_capture(&mut self, ctx: &Context) {
        match self.state.capture_state {
            state::CaptureState::Minimising { frames_waited } => {
                if frames_waited == 0 {
                    ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                }

                if frames_waited >= MINIMISE_WAIT_FRAMES {
                    // Give the OS a moment to finish the minimise animation.
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    screenshot::capture_and_load(ctx, &mut self.state);
                    ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
                    self.state.capture_state = state::CaptureState::Restoring;
                } else {
                    self.state.capture_state = state::CaptureState::Minimising {
                        frames_waited: frames_waited + 1,
                    };
                    // Request another repaint so we keep ticking.
                    ctx.request_repaint();
                }
            }
            state::CaptureState::Restoring => {
                self.state.capture_state = state::CaptureState::Idle;
            }
            state::CaptureState::Idle => {}
        }
    }

    /// Captures the canvas area and copies it to the system clipboard.
    fn copy_canvas_to_clipboard(&self) {
        let Some(rect) = self.canvas.canvas_rect() else {
            return;
        };

        // Use xcap to capture the canvas region
        let monitors = match xcap::Monitor::all() {
            Ok(m) => m,
            Err(_err) => {
                #[cfg(debug_assertions)]
                eprintln!("clipboard copy: failed to list monitors: {_err}");
                return;
            }
        };

        let Some(monitor) = monitors.into_iter().next() else {
            return;
        };

        let screenshot = match monitor.capture_image() {
            Ok(img) => img,
            Err(_err) => {
                #[cfg(debug_assertions)]
                eprintln!("clipboard copy: failed to capture screen: {_err}");
                return;
            }
        };

        // Crop to the canvas rect (screen coordinates)
        let x = (rect.min.x as u32).min(screenshot.width().saturating_sub(1));
        let y = (rect.min.y as u32).min(screenshot.height().saturating_sub(1));
        let w = (rect.width() as u32).min(screenshot.width().saturating_sub(x));
        let h = (rect.height() as u32).min(screenshot.height().saturating_sub(y));

        if w == 0 || h == 0 {
            return;
        }

        let cropped = image::imageops::crop_imm(&screenshot, x, y, w, h).to_image();
        clipboard::copy_to_clipboard(cropped.as_raw(), w as usize, h as usize);
    }

    /// Reads an image from the clipboard and adds it as a DrawObject at the canvas centre.
    fn paste_image_from_clipboard(&mut self, ctx: &Context) {
        let Some((rgba, width, height)) = clipboard::paste_from_clipboard() else {
            return;
        };

        if width == 0 || height == 0 {
            return;
        }

        let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba);
        let texture =
            ctx.load_texture("clipboard_paste", color_image, egui::TextureOptions::LINEAR);

        // Place image centred in the canvas using normalised coordinates.
        // Use a sensible default size relative to the canvas (e.g. 0.4 of canvas width).
        let canvas_rect = self.canvas.canvas_rect();
        let (norm_pos, norm_size) = if let Some(rect) = canvas_rect {
            let aspect = height as f32 / width as f32;
            let norm_w = 0.4_f32; // 40% of canvas normalised width
            let norm_h = norm_w * aspect * (rect.width() / rect.height());
            let cx = 0.5 - norm_w / 2.0;
            let cy = 0.5 - norm_h / 2.0;
            (egui::pos2(cx, cy), egui::vec2(norm_w, norm_h))
        } else {
            (egui::pos2(0.3, 0.3), egui::vec2(0.4, 0.4))
        };

        self.state.objects.push(state::DrawObject::Image {
            texture,
            pos: norm_pos,
            size: norm_size,
        });
    }
}
