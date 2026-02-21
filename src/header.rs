use egui::Layout;

use crate::state::AppState;

pub struct Header {
    /// Whether the theme toggle was clicked this frame
    theme_toggled: bool,
}

impl Header {
    pub fn new() -> Self {
        Self {
            theme_toggled: false,
        }
    }

    /// Returns true if the theme toggle button was clicked since the last call
    pub fn take_theme_toggled(&mut self) -> bool {
        std::mem::take(&mut self.theme_toggled)
    }
}

impl super::View for Header {
    fn render(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        egui::MenuBar::new().ui(ui, |ui| {
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
                let is_dark = ui.visuals().dark_mode;
                // Show the icon for the opposite theme
                let icon = if is_dark { "☀" } else { "🌙" };
                let tooltip = if is_dark {
                    "Switch to light theme"
                } else {
                    "Switch to dark theme"
                };
                if ui.button(icon).on_hover_text(tooltip).clicked() {
                    self.theme_toggled = true;
                }

                if ui
                    .button("Export")
                    .on_hover_text("Export canvas as PNG")
                    .clicked()
                {
                    state.export_requested = true;
                }
            });
        });
    }
}
