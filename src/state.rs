use egui::{Color32, Pos2, Rect, Vec2};

use crate::history::History;

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

impl DrawObject {
    /// Returns the axis-aligned bounding rectangle of this object in normalised coordinates.
    pub fn bounding_rect(&self) -> Option<Rect> {
        match self {
            DrawObject::Freehand { points, .. } => {
                if points.is_empty() {
                    return None;
                }
                let mut min = points[0];
                let mut max = points[0];
                for p in points.iter().skip(1) {
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                }
                Some(Rect::from_min_max(min, max))
            }
            DrawObject::Rectangle { min, max, .. } => Some(Rect::from_min_max(*min, *max)),
            DrawObject::Ellipse {
                center,
                radius_x,
                radius_y,
                ..
            } => Some(Rect::from_center_size(
                *center,
                Vec2::new(radius_x * 2.0, radius_y * 2.0),
            )),
            DrawObject::Line { start, end, .. } | DrawObject::Arrow { start, end, .. } => {
                let min = Pos2::new(start.x.min(end.x), start.y.min(end.y));
                let max = Pos2::new(start.x.max(end.x), start.y.max(end.y));
                Some(Rect::from_min_max(min, max))
            }
            DrawObject::Text { pos, .. } => {
                let size = 0.05;
                Some(Rect::from_min_size(*pos, Vec2::new(size, size)))
            }
            DrawObject::Image { pos, size } => Some(Rect::from_min_size(*pos, *size)),
        }
    }

    /// Moves the object by the given delta in normalised coordinates.
    pub fn offset_by(&mut self, delta: Vec2) {
        match self {
            DrawObject::Freehand { points, .. } => {
                for p in points.iter_mut() {
                    *p += delta;
                }
            }
            DrawObject::Rectangle { min, max, .. } => {
                *min += delta;
                *max += delta;
            }
            DrawObject::Ellipse { center, .. } => {
                *center += delta;
            }
            DrawObject::Line { start, end, .. } | DrawObject::Arrow { start, end, .. } => {
                *start += delta;
                *end += delta;
            }
            DrawObject::Text { pos, .. } => {
                *pos += delta;
            }
            DrawObject::Image { pos, .. } => {
                *pos += delta;
            }
        }
    }
}

/// In-progress text being edited on the canvas.
#[derive(Debug, Clone)]
pub struct TextEdit {
    /// Position in normalised 0..1 coordinates.
    pub position: Pos2,
    /// The text content being typed.
    pub content: String,
    /// Colour captured when editing started.
    pub colour: Color32,
    /// Font size captured when editing started.
    pub font_size: f32,
}

/// Shared application state passed to all components each frame.
pub struct AppState {
    pub active_tool: Tool,
    pub active_colour: Color32,
    pub stroke_width: f32,
    pub objects: Vec<DrawObject>,
    /// The freehand stroke currently being drawn (not yet committed to objects).
    pub current_stroke: Option<Vec<Pos2>>,
    /// Text annotation currently being edited on the canvas.
    pub editing_text: Option<TextEdit>,
    /// Undo/redo history for canvas operations.
    pub history: History,
    /// Drag start position (normalised 0..1) for shape tools.
    pub shape_start: Option<Pos2>,
    /// Set to true when the user clicks the export button; consumed by the app loop.
    pub export_requested: bool,
    /// Index of the currently selected object (for the Selection tool).
    pub selected_index: Option<usize>,
    /// Offset between pointer and object origin when dragging a selected object.
    pub drag_offset: Option<Vec2>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tool: Tool::Freehand,
            active_colour: Color32::from_rgb(0, 0, 0),
            stroke_width: 2.0,
            objects: Vec::new(),
            current_stroke: None,
            editing_text: None,
            history: History::new(),
            shape_start: None,
            export_requested: false,
            selected_index: None,
            drag_offset: None,
        }
    }
}
