use egui::{Color32, Pos2, TextureHandle};

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
#[derive(Clone)]
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
        texture: TextureHandle,
        pos: Pos2,
        size: egui::Vec2,
    },
}

impl std::fmt::Debug for DrawObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Freehand {
                points,
                colour,
                width,
            } => f
                .debug_struct("Freehand")
                .field("points", points)
                .field("colour", colour)
                .field("width", width)
                .finish(),
            Self::Rectangle {
                min,
                max,
                colour,
                width,
            } => f
                .debug_struct("Rectangle")
                .field("min", min)
                .field("max", max)
                .field("colour", colour)
                .field("width", width)
                .finish(),
            Self::Ellipse {
                center,
                radius_x,
                radius_y,
                colour,
                width,
            } => f
                .debug_struct("Ellipse")
                .field("center", center)
                .field("radius_x", radius_x)
                .field("radius_y", radius_y)
                .field("colour", colour)
                .field("width", width)
                .finish(),
            Self::Line {
                start,
                end,
                colour,
                width,
            } => f
                .debug_struct("Line")
                .field("start", start)
                .field("end", end)
                .field("colour", colour)
                .field("width", width)
                .finish(),
            Self::Arrow {
                start,
                end,
                colour,
                width,
            } => f
                .debug_struct("Arrow")
                .field("start", start)
                .field("end", end)
                .field("colour", colour)
                .field("width", width)
                .finish(),
            Self::Text {
                pos,
                content,
                font_size,
                colour,
            } => f
                .debug_struct("Text")
                .field("pos", pos)
                .field("content", content)
                .field("font_size", font_size)
                .field("colour", colour)
                .finish(),
            Self::Image { pos, size, .. } => f
                .debug_struct("Image")
                .field("pos", pos)
                .field("size", size)
                .finish_non_exhaustive(),
        }
    }
}

/// Tracks the screenshot capture state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureState {
    /// No capture in progress.
    Idle,
    /// Window minimise requested; waiting for it to disappear.
    Minimising { frames_waited: u32 },
    /// Capture taken; window restore requested.
    Restoring,
}

/// Shared application state passed to all components each frame.
pub struct AppState {
    pub active_tool: Tool,
    pub active_colour: Color32,
    pub stroke_width: f32,
    pub objects: Vec<DrawObject>,
    /// The freehand stroke currently being drawn (not yet committed to objects).
    pub current_stroke: Option<Vec<Pos2>>,
    /// Screenshot capture state machine.
    pub capture_state: CaptureState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tool: Tool::Freehand,
            active_colour: Color32::from_rgb(0, 0, 0),
            stroke_width: 2.0,
            objects: Vec::new(),
            current_stroke: None,
            capture_state: CaptureState::Idle,
        }
    }
}
