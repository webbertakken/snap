use egui::{Button, Color32};

use crate::{center_widget, palette::Palette};

pub struct Footer {
    center_widget: center_widget::Widget,
    palette: Palette,
}

impl Footer {
    pub fn new() -> Self {
        Self {
            center_widget: center_widget::Widget::new(),
            palette: Palette::new(),
        }
    }
}

impl super::View for Footer {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.center_widget.update(ui, |ui| {
            let color_button = |title: &str, color: Color32| {
                Button::new(title)
                    .fill(color)
                    .min_size(egui::Vec2 { x: 48.0, y: 48.0 })
            };

            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    for i in vec!["🖊️", "✒️", "✏️"] {
                        if ui.add(color_button(i, Color32::from_gray(8))).clicked() {
                            println!("Tool {}  chosen", i);
                        }
                    }

                    ui.separator();

                    for i in 0..10 {
                        let id = format!("");
                        let color = self.palette.get_color(i).unwrap();
                        if ui.add(color_button(&id, color)).clicked() {
                            println!("Picked colour {}", i)
                        }

                        if i % 2 == 1 {
                            ui.add_space(8.0)
                        }
                    }

                    ui.separator();

                    if ui.add(color_button("E", Color32::from_gray(8))).clicked() {
                        println!("Eraser chosen");
                    }
                });
            });
        })
    }
}
