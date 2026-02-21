use egui::Pos2;

use crate::state::DrawObject;

/// Radius (in normalised 0..1 coordinates) within which the eraser removes objects.
const ERASER_TOLERANCE: f32 = 0.02;

/// Screen-space radius of the eraser cursor circle.
pub const ERASER_CURSOR_RADIUS: f32 = 12.0;

/// Returns true if the given normalised position is close enough to the object to erase it.
pub fn hit_test(object: &DrawObject, pos: Pos2) -> bool {
    match object {
        DrawObject::Freehand { points, width, .. } => {
            // Use a tolerance proportional to the stroke width, with a minimum
            let tolerance = ERASER_TOLERANCE + (*width * 0.002);
            distance_to_polyline(pos, points) < tolerance
        }
        DrawObject::Rectangle { min, max, .. } => {
            let rect = egui::Rect::from_min_max(*min, *max);
            rect.expand(ERASER_TOLERANCE).contains(pos)
        }
        DrawObject::Ellipse {
            center,
            radius_x,
            radius_y,
            ..
        } => {
            let dx = pos.x - center.x;
            let dy = pos.y - center.y;
            // Normalised distance from ellipse center (< 1.0 means inside)
            let r = *radius_x + ERASER_TOLERANCE;
            let ry = *radius_y + ERASER_TOLERANCE;
            if r <= 0.0 || ry <= 0.0 {
                return false;
            }
            (dx * dx) / (r * r) + (dy * dy) / (ry * ry) <= 1.0
        }
        DrawObject::Line { start, end, .. } | DrawObject::Arrow { start, end, .. } => {
            distance_to_segment(pos, *start, *end) < ERASER_TOLERANCE
        }
        DrawObject::Text { pos: text_pos, .. } => {
            // Simple bounding-box approximation
            let size = 0.05; // rough text bounding size in normalised coords
            let rect = egui::Rect::from_min_size(*text_pos, egui::vec2(size, size));
            rect.expand(ERASER_TOLERANCE).contains(pos)
        }
        DrawObject::Image { pos: img_pos, size } => {
            let rect = egui::Rect::from_min_size(*img_pos, *size);
            rect.expand(ERASER_TOLERANCE).contains(pos)
        }
    }
}

/// Minimum distance from a point to a polyline (sequence of connected segments).
fn distance_to_polyline(point: Pos2, polyline: &[Pos2]) -> f32 {
    if polyline.is_empty() {
        return f32::MAX;
    }
    if polyline.len() == 1 {
        return point.distance(polyline[0]);
    }

    polyline
        .windows(2)
        .map(|seg| distance_to_segment(point, seg[0], seg[1]))
        .fold(f32::MAX, f32::min)
}

/// Minimum distance from a point to a line segment.
fn distance_to_segment(point: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let ap = point - a;
    let len_sq = ab.length_sq();

    if len_sq == 0.0 {
        return ap.length();
    }

    // Project point onto the segment, clamped to [0, 1]
    let t = (ap.x * ab.x + ap.y * ab.y) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let closest = Pos2::new(a.x + t * ab.x, a.y + t * ab.y);
    point.distance(closest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Color32, Pos2};

    #[test]
    fn hit_test_detects_close_freehand() {
        let obj = DrawObject::Freehand {
            points: vec![Pos2::new(0.5, 0.5), Pos2::new(0.6, 0.5)],
            colour: Color32::BLACK,
            width: 2.0,
        };
        // Point right on the line
        assert!(hit_test(&obj, Pos2::new(0.55, 0.5)));
        // Point far away
        assert!(!hit_test(&obj, Pos2::new(0.0, 0.0)));
    }

    #[test]
    fn hit_test_detects_rectangle() {
        let obj = DrawObject::Rectangle {
            min: Pos2::new(0.2, 0.2),
            max: Pos2::new(0.8, 0.8),
            colour: Color32::RED,
            width: 2.0,
        };
        assert!(hit_test(&obj, Pos2::new(0.5, 0.5)));
        assert!(!hit_test(&obj, Pos2::new(0.0, 0.0)));
    }

    #[test]
    fn distance_to_segment_on_point() {
        let d = distance_to_segment(
            Pos2::new(0.5, 0.5),
            Pos2::new(0.5, 0.5),
            Pos2::new(1.0, 0.5),
        );
        assert!(d < 0.001);
    }

    #[test]
    fn distance_to_segment_perpendicular() {
        let d = distance_to_segment(
            Pos2::new(0.5, 0.6),
            Pos2::new(0.0, 0.5),
            Pos2::new(1.0, 0.5),
        );
        assert!((d - 0.1).abs() < 0.001);
    }
}
