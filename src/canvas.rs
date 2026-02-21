use egui::*;

use crate::eraser;
use crate::history::Command;
use crate::state::{AppState, DrawObject, Tool};

pub struct Canvas;

impl Canvas {
    pub fn new() -> Self {
        Self
    }

    fn ui_content(&self, ui: &mut Ui, state: &mut AppState) -> egui::Response {
        let (mut response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());

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
            Tool::Rectangle | Tool::Ellipse | Tool::Line | Tool::Arrow => {
                self.handle_shape_input(&mut response, state, &from_screen);
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

        // Render shape preview during drag
        if let Some(start) = state.shape_start {
            if let Some(pointer_pos) = response.hover_pos() {
                let current = from_screen * pointer_pos;
                let preview_colour = {
                    let [r, g, b, _] = state.active_colour.to_array();
                    Color32::from_rgba_unmultiplied(r, g, b, 160)
                };
                if let Some(preview) = self.build_shape_object(
                    state.active_tool,
                    start,
                    current,
                    preview_colour,
                    state.stroke_width,
                ) {
                    if let Some(shape) = self.render_object(&preview, &to_screen) {
                        painter.add(shape);
                    }
                }
            }
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
                let obj = DrawObject::Freehand {
                    points,
                    colour: state.active_colour,
                    width: state.stroke_width,
                };
                state.objects.push(obj.clone());
                state.history.push(Command::Add(obj));
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
            // Collect indices of hit objects in reverse order so removals don't shift later indices
            let hit_indices: Vec<usize> = state
                .objects
                .iter()
                .enumerate()
                .filter(|(_, obj)| eraser::hit_test(obj, canvas_pos))
                .map(|(i, _)| i)
                .rev()
                .collect();

            for index in hit_indices {
                let removed = state.objects.remove(index);
                state.history.push(Command::Remove(index, removed));
            }
        }
    }

    fn handle_shape_input(
        &self,
        response: &mut Response,
        state: &mut AppState,
        from_screen: &emath::RectTransform,
    ) {
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = *from_screen * pointer_pos;
            if state.shape_start.is_none() {
                state.shape_start = Some(canvas_pos);
            }
            response.mark_changed();
        } else if let Some(start) = state.shape_start.take() {
            // Pointer released — commit the shape
            if let Some(hover) = response.hover_pos() {
                let end = *from_screen * hover;
                if let Some(obj) = self.build_shape_object(
                    state.active_tool,
                    start,
                    end,
                    state.active_colour,
                    state.stroke_width,
                ) {
                    state.objects.push(obj);
                }
            }
            response.mark_changed();
        }
    }

    fn build_shape_object(
        &self,
        tool: Tool,
        start: Pos2,
        end: Pos2,
        colour: Color32,
        width: f32,
    ) -> Option<DrawObject> {
        match tool {
            Tool::Rectangle => Some(DrawObject::Rectangle {
                min: Pos2::new(start.x.min(end.x), start.y.min(end.y)),
                max: Pos2::new(start.x.max(end.x), start.y.max(end.y)),
                colour,
                width,
            }),
            Tool::Ellipse => {
                let center = Pos2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
                let radius_x = (end.x - start.x).abs() / 2.0;
                let radius_y = (end.y - start.y).abs() / 2.0;
                Some(DrawObject::Ellipse {
                    center,
                    radius_x,
                    radius_y,
                    colour,
                    width,
                })
            }
            Tool::Line => Some(DrawObject::Line {
                start,
                end,
                colour,
                width,
            }),
            Tool::Arrow => Some(DrawObject::Arrow {
                start,
                end,
                colour,
                width,
            }),
            _ => None,
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
