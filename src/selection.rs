use egui::{emath, Color32, Key, Pos2, Stroke, Vec2};

use crate::eraser;
use crate::state::{AppState, DrawObject};

/// Selection hit-test tolerance in normalised coordinates.
const SELECTION_TOLERANCE: f32 = 0.02;

/// Colour of the selection bounding box.
const SELECTION_COLOUR: Color32 = Color32::from_rgb(100, 160, 255);

/// Padding around the selection bounding box in normalised coordinates.
const BBOX_PADDING: f32 = 0.005;

/// Returns the index of the topmost object hit at the given normalised position,
/// iterating in reverse (topmost first).
pub fn find_object_at(objects: &[DrawObject], pos: Pos2) -> Option<usize> {
    for (i, obj) in objects.iter().enumerate().rev() {
        if selection_hit_test(obj, pos) {
            return Some(i);
        }
    }
    None
}

/// Hit-test an object for selection purposes (slightly more generous than eraser).
fn selection_hit_test(object: &DrawObject, pos: Pos2) -> bool {
    match object {
        DrawObject::Freehand { points, width, .. } => {
            let tolerance = SELECTION_TOLERANCE + (*width * 0.002);
            eraser::distance_to_polyline(pos, points) < tolerance
        }
        DrawObject::Rectangle { min, max, .. } => {
            let rect = egui::Rect::from_min_max(*min, *max);
            rect.expand(SELECTION_TOLERANCE).contains(pos)
        }
        DrawObject::Ellipse {
            center,
            radius_x,
            radius_y,
            ..
        } => {
            let dx = pos.x - center.x;
            let dy = pos.y - center.y;
            let r = *radius_x + SELECTION_TOLERANCE;
            let ry = *radius_y + SELECTION_TOLERANCE;
            if r <= 0.0 || ry <= 0.0 {
                return false;
            }
            (dx * dx) / (r * r) + (dy * dy) / (ry * ry) <= 1.0
        }
        DrawObject::Line { start, end, .. } | DrawObject::Arrow { start, end, .. } => {
            eraser::distance_to_segment(pos, *start, *end) < SELECTION_TOLERANCE
        }
        DrawObject::Text { pos: text_pos, .. } => {
            let size = 0.05;
            let rect = egui::Rect::from_min_size(*text_pos, egui::vec2(size, size));
            rect.expand(SELECTION_TOLERANCE).contains(pos)
        }
        DrawObject::Image { pos: img_pos, size } => {
            let rect = egui::Rect::from_min_size(*img_pos, *size);
            rect.expand(SELECTION_TOLERANCE).contains(pos)
        }
    }
}

/// Handles selection tool input: click to select, drag to move, Delete to remove.
pub fn handle_selection_input(
    response: &egui::Response,
    state: &mut AppState,
    from_screen: &emath::RectTransform,
    ctx: &egui::Context,
) {
    // Handle Delete/Backspace to remove selected object
    if state.selected_index.is_some() {
        let delete_pressed =
            ctx.input(|i| i.key_pressed(Key::Delete) || i.key_pressed(Key::Backspace));
        if delete_pressed {
            if let Some(idx) = state.selected_index.take() {
                if idx < state.objects.len() {
                    state.objects.remove(idx);
                }
            }
            state.drag_offset = None;
            return;
        }
    }

    // Handle pointer interactions
    if let Some(pointer_pos) = response.interact_pointer_pos() {
        let canvas_pos = *from_screen * pointer_pos;

        if response.drag_started() {
            // Starting a new click/drag: try to select an object
            if let Some(idx) = find_object_at(&state.objects, canvas_pos) {
                state.selected_index = Some(idx);
                // Calculate offset from pointer to the object's bounding rect min
                if let Some(bbox) = state.objects[idx].bounding_rect() {
                    let offset = Vec2::new(canvas_pos.x - bbox.min.x, canvas_pos.y - bbox.min.y);
                    state.drag_offset = Some(offset);
                }
            } else {
                // Clicked on empty area: clear selection
                state.selected_index = None;
                state.drag_offset = None;
            }
        } else if response.dragged() {
            // Dragging: move the selected object
            if let (Some(idx), Some(_offset)) = (state.selected_index, state.drag_offset) {
                if idx < state.objects.len() {
                    let delta = response.drag_delta();
                    // Convert screen delta to normalised coordinates
                    let from_rect = from_screen.to(); // screen rect
                    let to_rect = from_screen.from(); // normalised rect
                    let scale_x = to_rect.width() / from_rect.width();
                    let scale_y = to_rect.height() / from_rect.height();
                    let normalised_delta = Vec2::new(delta.x * scale_x, delta.y * scale_y);
                    state.objects[idx].offset_by(normalised_delta);
                }
            }
        }
    } else {
        // Pointer released: clear drag offset but keep selection
        state.drag_offset = None;
    }
}

/// Draws a dashed bounding box around the selected object.
pub fn draw_selection_box(
    painter: &egui::Painter,
    state: &AppState,
    to_screen: &emath::RectTransform,
) {
    let idx = match state.selected_index {
        Some(i) if i < state.objects.len() => i,
        _ => return,
    };

    let bbox = match state.objects[idx].bounding_rect() {
        Some(r) => r,
        None => return,
    };

    // Expand bbox slightly and transform to screen space
    let padded = bbox.expand(BBOX_PADDING);
    let screen_min = *to_screen * padded.min;
    let screen_max = *to_screen * padded.max;
    let screen_rect = egui::Rect::from_min_max(screen_min, screen_max);

    let stroke = Stroke::new(1.5, SELECTION_COLOUR);
    let dash_length = 5.0;
    let gap_length = 4.0;

    // Draw dashed rectangle as four dashed sides
    let corners = [
        screen_rect.left_top(),
        screen_rect.right_top(),
        screen_rect.right_bottom(),
        screen_rect.left_bottom(),
    ];

    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        draw_dashed_line(painter, a, b, stroke, dash_length, gap_length);
    }
}

/// Draws a dashed line between two points.
fn draw_dashed_line(
    painter: &egui::Painter,
    a: Pos2,
    b: Pos2,
    stroke: Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    let total_length = a.distance(b);
    if total_length < 0.1 {
        painter.line_segment([a, b], stroke);
        return;
    }

    let dir = (b - a) / total_length;
    let mut pos = 0.0;
    let mut drawing = true;

    while pos < total_length {
        let segment_len = if drawing { dash_length } else { gap_length };
        let end = (pos + segment_len).min(total_length);

        if drawing {
            let start_pt = a + dir * pos;
            let end_pt = a + dir * end;
            painter.line_segment([start_pt, end_pt], stroke);
        }

        pos = end;
        drawing = !drawing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Color32;

    #[test]
    fn find_object_at_returns_topmost_hit() {
        let objects = vec![
            DrawObject::Freehand {
                points: vec![Pos2::new(0.5, 0.5), Pos2::new(0.6, 0.5)],
                colour: Color32::BLACK,
                width: 2.0,
            },
            DrawObject::Rectangle {
                min: Pos2::new(0.4, 0.4),
                max: Pos2::new(0.7, 0.7),
                colour: Color32::RED,
                width: 2.0,
            },
        ];

        // Point inside the rectangle (index 1) which is topmost
        let result = find_object_at(&objects, Pos2::new(0.55, 0.5));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn find_object_at_returns_none_for_empty_area() {
        let objects = vec![DrawObject::Freehand {
            points: vec![Pos2::new(0.5, 0.5), Pos2::new(0.6, 0.5)],
            colour: Color32::BLACK,
            width: 2.0,
        }];

        let result = find_object_at(&objects, Pos2::new(0.0, 0.0));
        assert_eq!(result, None);
    }

    #[test]
    fn find_object_at_hits_freehand() {
        let objects = vec![DrawObject::Freehand {
            points: vec![Pos2::new(0.5, 0.5), Pos2::new(0.6, 0.5)],
            colour: Color32::BLACK,
            width: 2.0,
        }];

        let result = find_object_at(&objects, Pos2::new(0.55, 0.5));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn find_object_at_hits_line() {
        let objects = vec![DrawObject::Line {
            start: Pos2::new(0.2, 0.2),
            end: Pos2::new(0.8, 0.8),
            colour: Color32::BLUE,
            width: 2.0,
        }];

        let result = find_object_at(&objects, Pos2::new(0.5, 0.5));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn find_object_at_hits_ellipse() {
        let objects = vec![DrawObject::Ellipse {
            center: Pos2::new(0.5, 0.5),
            radius_x: 0.1,
            radius_y: 0.1,
            colour: Color32::GREEN,
            width: 2.0,
        }];

        let result = find_object_at(&objects, Pos2::new(0.5, 0.5));
        assert_eq!(result, Some(0));
    }
}
