use super::{
    core::{Duration, Position, Samples, Scenario},
    import::{Point3, Vec3},
    misc::{entity_store, PointFormat},
    ui_import::{egui, Color32, Pos2, Vec2},
    Canvas, Integration,
};
use ::std::cell::{Ref, RefCell, RefMut};

pub struct Painter<'c> {
    canvas: RefMut<'c, Canvas>,
    response: egui::Response,
    painter: egui::Painter,
}

impl<'c> Painter<'c> {
    pub fn new(
        canvas: &'c RefCell<Canvas>,
        response: egui::Response,
        painter: egui::Painter,
    ) -> Self {
        // borrowing here makes further borrowing unnecessary, until the new Self gets dropped
        let mut canvas = canvas.borrow_mut();
        // this initialization is required before first rendering:
        canvas.adjust_scale_and_center(&response.rect);
        Self {
            canvas,
            response,
            painter,
        }
    }

    pub fn for_each_integration(&self, f: impl FnMut(Ref<'_, Integration>)) {
        self.canvas
            .integrations
            .iter()
            .map(RefCell::borrow)
            .for_each(f);
    }

    pub fn for_each_integration_mut(&self, f: impl FnMut(RefMut<'_, Integration>)) {
        self.canvas
            .integrations
            .iter()
            .map(RefCell::borrow_mut)
            .for_each(f);
    }

    pub fn map_integrations<B, F>(&self, f: F) -> ::std::vec::IntoIter<B>
    where
        Self: Sized,
        F: FnMut(RefMut<Integration>) -> B,
    {
        self.canvas
            .integrations
            .iter()
            .map(RefCell::borrow_mut)
            .map(f)
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn scenario_idx(&self) -> entity_store::Index<Scenario> {
        self.canvas.scenario_idx()
    }

    pub fn input(&self) -> &egui::InputState {
        self.response.ctx.input()
    }

    pub fn rect_min(&self) -> Point3 {
        self.canvas.screen_to_user(Pos2::new(
            self.response.rect.min.x,
            self.response.rect.max.y, // user coords go from bottom to top
        ))
    }

    pub fn rect_max(&self) -> Point3 {
        self.canvas.screen_to_user(Pos2::new(
            self.response.rect.max.x,
            self.response.rect.min.y, // user coords go from bottom to top
        ))
    }

    pub fn pan_and_zoom(&mut self) {
        let input = self.response.ctx.input();
        // todo: propose pull request to integrate the below check for `touch.start_pos` into
        // `response.hovered()`
        if self.response.hovered()
            || input
                .multi_touch()
                .map_or(false, |touch| self.response.rect.contains(touch.start_pos))
        {
            let zoom = input.zoom_delta();
            let translation = input
                .multi_touch()
                .map_or(input.scroll_delta, |touch| touch.translation_delta);

            #[allow(clippy::float_cmp)]
            if zoom != 1. || translation != Vec2::ZERO {
                self.canvas.visible_units /= zoom;
                let screen_focus = self.canvas.user_to_screen(self.canvas.focus);
                self.canvas.focus = self.canvas.screen_to_user(screen_focus - translation);

                self.canvas.adjust_scale_and_center(&self.response.rect);
            }
        }
    }

    pub fn draw_hline(&self, y: f32, stroke: egui::Stroke) {
        let transformed_y = self.canvas.user_to_screen(Point3::new(0., y, 0.)).y;
        self.painter.line_segment(
            [
                Pos2::new(self.response.rect.left(), transformed_y),
                Pos2::new(self.response.rect.right(), transformed_y),
            ],
            stroke,
        );
    }

    pub fn draw_vline(&self, x: f32, stroke: egui::Stroke) {
        let transformed_x = self.canvas.user_to_screen(Point3::new(x, 0., 0.)).x;
        self.painter.line_segment(
            [
                Pos2::new(transformed_x, self.response.rect.top()),
                Pos2::new(transformed_x, self.response.rect.bottom()),
            ],
            stroke,
        );
    }

    pub fn draw_line_segment(
        &self,
        start: impl Into<Point3>,
        end: impl Into<Point3>,
        stroke: egui::Stroke,
    ) {
        let canvas = &self.canvas;
        self.painter.line_segment(
            [canvas.user_to_screen(start), canvas.user_to_screen(end)],
            stroke,
        );
    }

    pub fn draw_sample_point(&self, p: impl Into<Point3>, format: &PointFormat) {
        format.draw_position_on(self.canvas.user_to_screen(p), &self.painter);
    }

    pub fn draw_sample_dots(&self, samples: &Samples, color: Color32, format: &PointFormat) {
        let canvas = &self.canvas;
        let mut adapted_format = (*format).clone();
        adapted_format.stroke.color = color;
        samples
            .step_positions()
            .map(|position| canvas.user_to_screen(position))
            .fold(Pos2::new(f32::MAX, f32::MAX), |u0, u1| {
                if (u0.x - u1.x).abs() > 1. || (u0.y - u1.y).abs() > 1. {
                    adapted_format.draw_position_on(u1, &self.painter);
                    u1
                } else {
                    u0
                }
            });
    }

    pub fn draw_vector(
        &self,
        start: impl Into<Point3>,
        vec: impl Into<Vec3>,
        stroke: egui::Stroke,
    ) {
        let (start, vec) = (start.into(), vec.into());
        let (start, end) = {
            (
                self.canvas.user_to_screen(start),
                self.canvas.user_to_screen(start + vec),
            )
        };
        let direction = end - start;
        let direction_normalized = direction.normalized();
        let mut tail = [Pos2::new(0., -2.), Pos2::new(0., 2.)];
        let mut tip = vec![Pos2::ZERO, Pos2::new(-6., -2.), Pos2::new(-6., 2.)];
        // TODO: replace by nalgebra transformation
        rotate(&mut tail, direction_normalized);
        rotate(&mut tip, direction_normalized);
        move_to(&mut tail, start);
        move_to(&mut tip, end);

        self.painter.line_segment(tail, stroke);
        self.painter.line_segment([start, end], stroke);
        self.painter
            .add(egui::Shape::convex_polygon(tip, stroke.color, stroke));
    }

    /// Execute `add_contents` when hovered, passing the mouse position translated to application
    /// coordinates.
    pub fn on_hover(&self, add_contents: impl FnOnce(Point3)) {
        let pointer = &self.response.ctx.input().pointer;
        if self.response.hovered() && pointer.has_pointer() {
            if let Some(mouse_pos) = pointer.hover_pos() {
                add_contents(self.canvas.screen_to_user(mouse_pos));
            }
        }
    }

    /// show a pop-up window if hovered
    pub fn on_hover_ui(&self, add_contents: impl FnOnce(&mut egui::Ui, Point3)) {
        let response = &self.response;
        if response.hovered() && response.ctx.input().pointer.has_pointer() {
            egui::popup::show_tooltip_at_pointer(
                &response.ctx,
                response.id.with("tooltip"),
                |ui| {
                    if let Some(mouse_pos) = ui.input().pointer.hover_pos() {
                        add_contents(ui, self.canvas.screen_to_user(mouse_pos));
                    }
                },
            );
        }
    }

    pub fn has_trajectory(&self) -> bool {
        self.canvas.has_trajectory()
    }

    pub fn update_trajectory(&mut self, scenario: &Scenario, min_dt: Duration) {
        self.canvas.update_trajectory(scenario, min_dt);
    }

    pub fn draw_trajectory(&self, stroke: egui::Stroke) {
        if let Some(ref buffer) = &self.canvas.trajectory_buffer {
            self.draw_connected_samples(buffer.iter().copied(), stroke);
        }
    }

    pub fn draw_sample_trajectory(&self, samples: &Samples, stroke: egui::Stroke) {
        if !samples.is_empty() {
            let start_position = samples.at(0).positions_iter().next();
            self.draw_connected_samples(
                start_position.into_iter().chain(samples.step_positions()),
                stroke,
            );
        }
    }

    fn draw_connected_samples(
        &self,
        positions: impl Iterator<Item = Position>,
        stroke: egui::Stroke,
    ) {
        let canvas = &self.canvas;
        positions
            .map(|p| canvas.user_to_screen(p))
            .reduce(|u0, u1| {
                // avoid drawing extremely short line segments:
                if (u0.x - u1.x).abs() > 2. || (u0.y - u1.y).abs() > 2. {
                    self.painter.line_segment([u0, u1], stroke);
                    u1
                } else {
                    u0
                }
            });
    }

    pub fn update_bounding_box(&mut self) {
        if let Some(mut bbox) = self.canvas.bbox() {
            self.canvas
                .integrations()
                .for_each(|integration| integration.borrow().stretch_bbox(&mut bbox));
            self.canvas.set_visible_bbox(&bbox);
        }
    }
}

fn move_to(positions: &mut [Pos2], translation: Pos2) {
    for mut p in positions {
        p.x += translation.x;
        p.y += translation.y;
    }
}

fn rotate(positions: &mut [Pos2], direction_normalized: Vec2) {
    let dir = direction_normalized;
    for mut p in positions {
        let tmp_x = p.x * dir.x - p.y * dir.y;
        p.y = p.x * dir.y + p.y * dir.x;
        p.x = tmp_x;
    }
}
