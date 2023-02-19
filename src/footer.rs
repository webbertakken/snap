use crate::center_widget;

//everything is handled by this struct
pub struct Widget {
    center_widget: center_widget::Widget,
}

//our methods for it
impl Widget {
    pub fn new() -> Self {
        Self {
            center_widget: center_widget::Widget::new(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        self.center_widget.update(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("button1");
                        ui.label("button2");
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("button3");
                        ui.label("button4");
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("button5");
                        ui.label("button6");
                    });
                });
            });
        })
    }
}
