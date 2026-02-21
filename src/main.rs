use std::sync::Arc;

use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::{
    CentralPanel, Context, FontData, FontDefinitions, FontFamily, SidePanel, TopBottomPanel,
};

mod canvas;
mod center_widget;
mod eraser;
mod footer;
mod header;
mod history;
mod palette;
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

impl App for Snap {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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
