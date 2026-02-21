use egui::{Button, Color32, Stroke, Vec2};

use crate::center_widget;
use crate::palette::Palette;
use crate::state::{AppState, Tool};

const BUTTON_SIZE: Vec2 = Vec2::new(48.0, 48.0);
/// Width of the selection indicator border.
const SELECTED_BORDER: f32 = 3.0;

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
    fn render(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        self.center_widget.update(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    // Freehand tool button
                    let freehand_btn = Button::new("\u{1f58a}\u{fe0f}")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Freehand))
                        .min_size(BUTTON_SIZE);
                    if ui.add(freehand_btn).clicked() {
                        state.active_tool = Tool::Freehand;
                    }

                    // Rectangle tool
                    let rect_btn = Button::new("\u{25ad}")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Rectangle))
                        .min_size(BUTTON_SIZE);
                    if ui.add(rect_btn).clicked() {
                        state.active_tool = Tool::Rectangle;
                    }

                    // Ellipse tool
                    let ellipse_btn = Button::new("\u{25cb}")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Ellipse))
                        .min_size(BUTTON_SIZE);
                    if ui.add(ellipse_btn).clicked() {
                        state.active_tool = Tool::Ellipse;
                    }

                    // Line tool
                    let line_btn = Button::new("\u{2571}")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Line))
                        .min_size(BUTTON_SIZE);
                    if ui.add(line_btn).clicked() {
                        state.active_tool = Tool::Line;
                    }

                    // Arrow tool
                    let arrow_btn = Button::new("\u{2192}")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Arrow))
                        .min_size(BUTTON_SIZE);
                    if ui.add(arrow_btn).clicked() {
                        state.active_tool = Tool::Arrow;
                    }

                    ui.separator();

                    // Colour palette buttons
                    for i in 0..10 {
                        if let Some(colour) = self.palette.get_color(i) {
                            let is_active =
                                state.active_colour == colour && state.active_tool != Tool::Eraser;
                            let btn = Button::new("")
                                .fill(colour)
                                .stroke(tool_border(is_active))
                                .min_size(BUTTON_SIZE);
                            if ui.add(btn).clicked() {
                                state.active_colour = colour;
                                // Switch back to freehand when picking a colour while erasing
                                if state.active_tool == Tool::Eraser {
                                    state.active_tool = Tool::Freehand;
                                }
                            }

                            if i % 2 == 1 {
                                ui.add_space(8.0);
                            }
                        }
                    }

                    ui.separator();

                    // Thickness preset buttons
                    for &thickness in &[1.0_f32, 2.0, 4.0, 8.0] {
                        let is_active = (state.stroke_width - thickness).abs() < f32::EPSILON;
                        let label = format!("{}px", thickness as u32);
                        let btn = Button::new(label)
                            .fill(Color32::from_gray(8))
                            .stroke(tool_border(is_active))
                            .min_size(BUTTON_SIZE);
                        if ui.add(btn).clicked() {
                            state.stroke_width = thickness;
                        }
                    }

                    ui.separator();

                    // Text tool button
                    let text_btn = Button::new("T")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Text))
                        .min_size(BUTTON_SIZE);
                    if ui.add(text_btn).clicked() {
                        state.active_tool = Tool::Text;
                    }

                    // Eraser button
                    let eraser_btn = Button::new("E")
                        .fill(Color32::from_gray(8))
                        .stroke(tool_border(state.active_tool == Tool::Eraser))
                        .min_size(BUTTON_SIZE);
                    if ui.add(eraser_btn).clicked() {
                        state.active_tool = Tool::Eraser;
                    }
                });
            });
        });
    }
}

/// Returns a highlighted stroke for selected buttons, or a subtle one otherwise.
fn tool_border(selected: bool) -> Stroke {
    if selected {
        Stroke::new(SELECTED_BORDER, Color32::WHITE)
    } else {
        Stroke::NONE
    }
}
