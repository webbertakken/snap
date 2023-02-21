use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::{
    CentralPanel, Context, FontData, FontDefinitions, FontFamily, SidePanel, TopBottomPanel,
};

mod center_widget;
mod footer;
mod header;
mod palette;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        initial_window_size: Some(egui::vec2(1680.0, 1050.0)),
        ..Default::default()
    };

    run_native(
        "Snap",
        native_options,
        Box::new(|cc| Box::new(Snap::new().init(cc))),
    )
}

struct Snap {
    header: header::Widget,
    footer: footer::Widget,
}

impl Snap {
    pub fn new() -> Self {
        Self {
            footer: footer::Widget::new(),
            header: header::Widget::new(),
        }
    }

    pub fn init(self, cc: &CreationContext) -> Self {
        self.configure_fonts(&cc.egui_ctx);

        self
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGM".to_string(),
            FontData::from_static(include_bytes!("../assets/MesloLGM.ttf")),
        );

        font_def
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "MesloLGM".to_string());

        ctx.set_fonts(font_def);
    }
}

impl App for Snap {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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
