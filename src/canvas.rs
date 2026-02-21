use egui::*;

use crate::eraser;
use crate::history::Command;
use crate::state::{AppState, DrawObject, TextEdit, Tool};

pub struct Canvas;

impl Canvas {
    pub fn new() -> Self {
        Self
    }

    fn ui_content(&self, ui: &mut Ui, state: &mut AppState) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

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
            Tool::Text => {
                Self::handle_text_click(&response, state, &from_screen);
            }
            Tool::Rectangle | Tool::Ellipse | Tool::Line | Tool::Arrow => {
                self.handle_shape_input(&mut response, state, &from_screen);
            }
            _ => {}
        }

        // Render all committed objects
        for obj in &state.objects {
            self.render_object_to_painter(obj, &to_screen, &painter);
        }

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

        // Render inline text editor
        Self::render_text_editor(ui, state, &to_screen, &from_screen);

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
                    self.render_object_to_painter(&preview, &to_screen, &painter);
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

    /// When the text tool is active, a click on the canvas starts a new text edit
    /// (committing any existing in-progress text first).
    fn handle_text_click(
        response: &Response,
        state: &mut AppState,
        from_screen: &emath::RectTransform,
    ) {
        if response.clicked() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                // Commit any existing in-progress text before starting a new one
                Self::commit_editing_text(state);

                let canvas_pos = *from_screen * pointer_pos;
                state.editing_text = Some(TextEdit {
                    position: canvas_pos,
                    content: String::new(),
                    colour: state.active_colour,
                    font_size: state.stroke_width * 6.0,
                });
            }
        }
    }

    /// Render the inline text editor widget at the editing position.
    fn render_text_editor(
        ui: &mut Ui,
        state: &mut AppState,
        to_screen: &emath::RectTransform,
        from_screen: &emath::RectTransform,
    ) {
        // Take editing_text out to avoid borrow conflicts
        let Some(mut editing) = state.editing_text.take() else {
            return;
        };

        let screen_pos = *to_screen * editing.position;
        let text_edit_id = Id::new("canvas_text_edit");

        let area_response = Area::new(text_edit_id)
            .fixed_pos(screen_pos)
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                let te = egui::TextEdit::singleline(&mut editing.content)
                    .font(FontId::proportional(editing.font_size))
                    .text_color(editing.colour)
                    .frame(false)
                    .desired_width(200.0)
                    .cursor_at_end(true);
                let te_response = ui.add(te);

                // Request focus on first frame
                if editing.content.is_empty() {
                    te_response.request_focus();
                }

                te_response
            });

        let te_response = area_response.inner;

        // Commit on Enter or Escape
        let enter_pressed = ui.input(|i| i.key_pressed(Key::Enter));
        let escape_pressed = ui.input(|i| i.key_pressed(Key::Escape));

        // Commit on click-away: the text edit lost focus and a click happened elsewhere
        let clicked_away = te_response.lost_focus()
            && ui.input(|i| i.pointer.any_click())
            && !te_response.contains_pointer();

        // Also check if a click happened outside the text edit area on the canvas
        let clicked_canvas_elsewhere =
            if let Some(click_pos) = ui.input(|i| i.pointer.press_origin()) {
                let canvas_click = *from_screen * click_pos;
                // Only if this is a new click, not the original placement click
                !editing.content.is_empty()
                    && ui.input(|i| i.pointer.any_pressed())
                    && canvas_click != editing.position
                    && !area_response.response.rect.contains(click_pos)
            } else {
                false
            };

        if enter_pressed || escape_pressed || clicked_away || clicked_canvas_elsewhere {
            // Commit non-empty text
            if !editing.content.trim().is_empty() {
                state.objects.push(DrawObject::Text {
                    pos: editing.position,
                    content: editing.content,
                    font_size: editing.font_size,
                    colour: editing.colour,
                });
            }
            // editing_text stays None (we already took it out)
        } else {
            // Keep editing
            state.editing_text = Some(editing);
        }
    }

    /// Commit any in-progress text edit to the objects list.
    fn commit_editing_text(state: &mut AppState) {
        if let Some(editing) = state.editing_text.take() {
            if !editing.content.trim().is_empty() {
                state.objects.push(DrawObject::Text {
                    pos: editing.position,
                    content: editing.content,
                    font_size: editing.font_size,
                    colour: editing.colour,
                });
            }
        }
    }

    /// Render a draw object to the painter. Text objects use `painter.text()` which
    /// doesn't return a `Shape`, so we render all objects directly via the painter.
    fn render_object_to_painter(
        &self,
        obj: &DrawObject,
        to_screen: &emath::RectTransform,
        painter: &Painter,
    ) {
        if let Some(shape) = self.render_object(obj, to_screen) {
            painter.add(shape);
        }
        // Handle types that render directly via painter (not returning Shape)
        if let DrawObject::Text {
            pos,
            content,
            font_size,
            colour,
        } = obj
        {
            let screen_pos = *to_screen * *pos;
            painter.text(
                screen_pos,
                Align2::LEFT_TOP,
                content,
                FontId::proportional(*font_size),
                *colour,
            );
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
            // Text is rendered via painter.text() in render_object_to_painter
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
