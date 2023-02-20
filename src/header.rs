pub struct Widget {}

impl Widget {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, ui: &mut egui::Ui) {
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
    }
}
