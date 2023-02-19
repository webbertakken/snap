use eframe::{run_native, App, NativeOptions};
use egui::{CentralPanel, SidePanel, TopBottomPanel};

struct Snap;

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

impl Default for Snap {
    fn default() -> Self {
        Self {}
    }
}

impl App for Snap {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel")
            .exact_height(64.0)
            .show(ctx, |ui| {
                ui.label("top");
            });

        TopBottomPanel::bottom("bottom_panel")
            .exact_height(64.0)
            .show(ctx, |ui| {
                ui.label("bottom");
            });

        SidePanel::left("left_panel")
            .exact_width(64.0)
            .show(ctx, |ui| ui.label("left"));

        CentralPanel::default().show(ctx, |ui| ui.label("center"));
    }
}
