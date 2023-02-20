use egui::Color32;

pub struct Palette {
    pub color0: Color32,
    pub color1: Color32,
    pub color2: Color32,
    pub color3: Color32,
    pub color4: Color32,
    pub color5: Color32,
    pub color6: Color32,
    pub color7: Color32,
    pub color8: Color32,
    pub color9: Color32,
}

impl Palette {
    // https://colorbox.io/
    pub fn new() -> Palette {
        Self {
            // Blacks
            color0: Color32::from_rgb(0, 0, 0),
            color1: Color32::from_rgb(50, 50, 50),
            // Blues
            color2: Color32::from_rgb(26, 26, 255),
            color3: Color32::from_rgb(0, 153, 255),
            // Greens
            color4: Color32::from_rgb(0, 102, 0),
            color5: Color32::from_rgb(128, 255, 128),
            // Yellows
            color7: Color32::from_rgb(255, 153, 0),
            color6: Color32::from_rgb(255, 255, 0),
            // Reds
            color8: Color32::from_rgb(204, 0, 0),
            color9: Color32::from_rgb(255, 0, 0),
        }
    }

    pub fn get_color(&self, i: u8) -> Option<Color32> {
        match i {
            0 => Some(self.color0),
            1 => Some(self.color1),
            2 => Some(self.color2),
            3 => Some(self.color3),
            4 => Some(self.color4),
            5 => Some(self.color5),
            6 => Some(self.color6),
            7 => Some(self.color7),
            8 => Some(self.color8),
            9 => Some(self.color9),
            _ => None,
        }
    }
}
