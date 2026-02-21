use egui::*;

use crate::eraser;
use crate::selection;
use crate::state::{AppState, DrawObject, Tool};

pub struct Canvas;

impl Canvas {
    pub fn new() -> Self {
        Self
    }

    fn ui_content(&self, ui: &mut Ui, state: &mut AppState) -> egui::Response {
        let sense = if state.active_tool == Tool::Selection {
            Sense::click_and_drag()
        } else {
            Sense::drag()
        };
        let (mut response, painter) = ui.allocate_painter(ui.available_size(), sense);

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        match state.active_tool {
            Tool::Freehand => {
                self.handle_freehand_input(&mut response, state, &from_screen);
            }
            Tool::Eraser => {
                self.handle_eraser_input(&response, state, &from_screen);
            }
            Tool::Selection => {
                selection::handle_selection_input(&response, state, &from_screen, ui.ctx());
            }
            _ => {}
        }

        // Render all committed objects
        let shapes: Vec<Shape> = state
            .objects
            .iter()
            .filter_map(|obj| self.render_object(obj, &to_screen))
            .collect();
        painter.extend(shapes);

        // Render the in-progress stroke
        if let Some(ref points) = state.current_stroke {
            if points.len() >= 2 {
                let screen_points: Vec<Pos2> = points.iter().map(|p| to_screen * *p).collect();
                painter.add(Shape::line(
                    screen_points,
                    Stroke::new(state.stroke_width, state.active_colour),
                ));
            }
        }

        // Draw selection bounding box
        if state.active_tool == Tool::Selection {
            selection::draw_selection_box(&painter, state, &to_screen);
        }

        // Draw eraser cursor
        if state.active_tool == Tool::Eraser {
            if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                painter.circle_stroke(
                    pos,
                    eraser::ERASER_CURSOR_RADIUS,
                    Stroke::new(1.5, Color32::from_gray(180)),
                );
            }
        }

        response
    }

    fn handle_freehand_input(
        &self,
        response: &mut Response,
        state: &mut AppState,
        from_screen: &emath::RectTransform,
    ) {
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = *from_screen * pointer_pos;

            let stroke = state.current_stroke.get_or_insert_with(Vec::new);
            if stroke.last() != Some(&canvas_pos) {
                stroke.push(canvas_pos);
                response.mark_changed();
            }
        } else if let Some(points) = state.current_stroke.take() {
            // Pointer released — commit the stroke as a DrawObject
            if points.len() >= 2 {
                state.objects.push(DrawObject::Freehand {
                    points,
                    colour: state.active_colour,
                    width: state.stroke_width,
                });
            }
            response.mark_changed();
        }
    }

    fn handle_eraser_input(
        &self,
        response: &Response,
        state: &mut AppState,
        from_screen: &emath::RectTransform,
    ) {
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = *from_screen * pointer_pos;
            // Remove objects the pointer touches (iterate in reverse so indices stay valid)
            state
                .objects
                .retain(|obj| !eraser::hit_test(obj, canvas_pos));
        }
    }

    fn render_object(&self, obj: &DrawObject, to_screen: &emath::RectTransform) -> Option<Shape> {
        match obj {
            DrawObject::Freehand {
                points,
                colour,
                width,
            } => {
                if points.len() < 2 {
                    return None;
                }
                let screen_points: Vec<Pos2> = points.iter().map(|p| *to_screen * *p).collect();
                Some(Shape::line(screen_points, Stroke::new(*width, *colour)))
            }
            DrawObject::Rectangle {
                min,
                max,
                colour,
                width,
            } => {
                let screen_min = *to_screen * *min;
                let screen_max = *to_screen * *max;
                Some(Shape::rect_stroke(
                    Rect::from_min_max(screen_min, screen_max),
                    CornerRadius::ZERO,
                    Stroke::new(*width, *colour),
                    StrokeKind::Outside,
                ))
            }
            DrawObject::Ellipse {
                center,
                radius_x,
                radius_y,
                colour,
                width,
            } => {
                let screen_center = *to_screen * *center;
                // Scale radii from normalised to screen space
                let scale = to_screen.to().size();
                let proportions = to_screen.from().size();
                let sx = scale.x / proportions.x;
                let sy = scale.y / proportions.y;
                Some(Shape::ellipse_stroke(
                    screen_center,
                    Vec2::new(radius_x * sx, radius_y * sy),
                    Stroke::new(*width, *colour),
                ))
            }
            DrawObject::Line {
                start,
                end,
                colour,
                width,
            } => {
                let a = *to_screen * *start;
                let b = *to_screen * *end;
                Some(Shape::line_segment([a, b], Stroke::new(*width, *colour)))
            }
            DrawObject::Arrow {
                start,
                end,
                colour,
                width,
            } => {
                let a = *to_screen * *start;
                let b = *to_screen * *end;
                // Simple arrow: line + arrowhead lines
                let dir = (b - a).normalized();
                let perp = Vec2::new(-dir.y, dir.x);
                let arrow_len = 10.0;
                let tip1 = b - arrow_len * dir + arrow_len * 0.4 * perp;
                let tip2 = b - arrow_len * dir - arrow_len * 0.4 * perp;
                let stroke = Stroke::new(*width, *colour);
                Some(Shape::Vec(vec![
                    Shape::line_segment([a, b], stroke),
                    Shape::line_segment([b, tip1], stroke),
                    Shape::line_segment([b, tip2], stroke),
                ]))
            }
            // Text and Image rendering are placeholder stubs
            DrawObject::Text { .. } | DrawObject::Image { .. } => None,
        }
    }
}

impl super::View for Canvas {
    fn render(&mut self, ui: &mut Ui, state: &mut AppState) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            self.ui_content(ui, state);
        });
    }
}
