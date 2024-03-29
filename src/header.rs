use egui::Layout;

pub struct Header {}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::View for Header {
    fn render(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("button1");
                    ui.label("button2");

                    ui.separator();

                    ui.label("button3");
                    ui.label("button4");

                    ui.separator();

                    ui.label("button5");
                    ui.label("button6");
                });
            });

            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("test");
            });
        });
    }
}
