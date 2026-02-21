use egui::{Color32, Pos2};

/// All available drawing/interaction tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Freehand,
    Rectangle,
    Ellipse,
    Line,
    Arrow,
    Text,
    Eraser,
    Selection,
}

/// A single canvas element with its own visual properties.
#[derive(Debug, Clone)]
pub enum DrawObject {
    Freehand {
        /// Points in normalised 0..1 coordinates.
        points: Vec<Pos2>,
        colour: Color32,
        width: f32,
    },
    // Placeholder variants for future shape tools
    Rectangle {
        min: Pos2,
        max: Pos2,
        colour: Color32,
        width: f32,
    },
    Ellipse {
        center: Pos2,
        radius_x: f32,
        radius_y: f32,
        colour: Color32,
        width: f32,
    },
    Line {
        start: Pos2,
        end: Pos2,
        colour: Color32,
        width: f32,
    },
    Arrow {
        start: Pos2,
        end: Pos2,
        colour: Color32,
        width: f32,
    },
    Text {
        pos: Pos2,
        content: String,
        font_size: f32,
        colour: Color32,
    },
    Image {
        pos: Pos2,
        size: egui::Vec2,
    },
}

/// Shared application state passed to all components each frame.
pub struct AppState {
    pub active_tool: Tool,
    pub active_colour: Color32,
    pub stroke_width: f32,
    pub objects: Vec<DrawObject>,
    /// The freehand stroke currently being drawn (not yet committed to objects).
    pub current_stroke: Option<Vec<Pos2>>,
    /// Drag start position (normalised 0..1) for shape tools.
    pub shape_start: Option<Pos2>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tool: Tool::Freehand,
            active_colour: Color32::from_rgb(0, 0, 0),
            stroke_width: 2.0,
            objects: Vec::new(),
            current_stroke: None,
            shape_start: None,
        }
    }
}
