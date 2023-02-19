use eframe::{run_native, App, NativeOptions};
use egui::{CentralPanel, SidePanel, TopBottomPanel};

mod center_widget;
mod footer;
mod header;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        initial_window_size: Some(egui::vec2(1680.0, 1050.0)),
        ..Default::default()
    };

    run_native(
        "Snap",
        native_options,
        Box::new(|_cc| Box::new(Snap::default())),
    )
}

struct Snap {
    header: header::Widget,
    footer: footer::Widget,
}

impl Default for Snap {
    fn default() -> Self {
        Self {
            footer: footer::Widget::new(),
            header: header::Widget::new(),
        }
    }
}

impl App for Snap {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel")
            .exact_height(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    self.header.update(ui);
                })
            });

        TopBottomPanel::bottom("bottom_panel")
            .exact_height(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    // ui.set_min_width(ui.available_width());
                    self.footer.update(ui);
                })
            });

        SidePanel::left("left_panel")
            .exact_width(64.0)
            .show_separator_line(false)
            .show(ctx, |ui| ui.label("left"));

        CentralPanel::default().show(ctx, |ui| ui.label("center"));
    }
}
