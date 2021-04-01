use crate::prelude::*;
use ::eframe::egui::{clamp, Painter, PointerButton, Response, Sense, Shape};

pub struct Kind;

pub mod comp {
    pub type State = super::State;
    pub type ScenarioId = crate::scenario::Entity;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

#[derive(bevy_ecs::Bundle)]
pub struct Bundle(pub Kind, pub State, pub comp::ScenarioId);

impl Bundle {
    pub fn spawn(self, world: &mut bevy_ecs::World) -> self::Entity {
        Entity(world.spawn(self))
    }
}

pub struct State {
    visible_units: f32,
    focus: Vec3,
    scale: Vec3,
    area_center: Pos2,
    trajectory: Vec<Vec3>,
    scenario_change_count: u32,
    trajectory_min_dt: R32,
    pub ui_integrations_window_is_open: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            visible_units: 1.,
            focus: Vec3::default(),
            trajectory: Vec::new(),
            scale: Vec3::default(),
            area_center: Pos2::default(),
            scenario_change_count: 0,
            trajectory_min_dt: R32::default(),
            ui_integrations_window_is_open: false,
        }
    }

    pub fn update_trajectory(
        &mut self,
        acceleration: &dyn AccelerationField,
        start_position: &ChangeTracker<Vec3, impl change_tracker::TRead>,
        start_velocity: &ChangeTracker<Vec3, impl change_tracker::TRead>,
        duration: &ChangeTracker<R32, impl change_tracker::TRead>,
        min_dt: R32,
    ) {
        let scenario_change_count =
            start_position.change_count() + start_velocity.change_count() + duration.change_count();
        if self.scenario_change_count != scenario_change_count || self.trajectory_min_dt > min_dt {
            self.trajectory = scenario::calculate_trajectory(
                acceleration,
                start_position,
                start_velocity,
                duration,
                min_dt,
            );
            self.trajectory_min_dt = min_dt;
            self.scenario_change_count = scenario_change_count;
        }
    }

    pub fn has_trajectory(&self) -> bool {
        !self.trajectory.is_empty()
    }

    pub fn reset_scenario(&mut self) {
        self.trajectory = Vec::new();
        self.scenario_change_count = 0;
    }

    pub fn bbox(&self) -> BoundingBox {
        let mut bbox = BoundingBox::default();
        self.trajectory.iter().for_each(|&s| bbox.expand_to(s));
        bbox
    }

    pub fn set_visible_bbox(&mut self, bbox: &BoundingBox) {
        self.focus = bbox.center();
        self.visible_units = bbox.diameter() * 1.2;
    }

    pub fn draw_sample_trajectory(&self, samples: &[Sample], stroke: Stroke, painter: &Painter) {
        self.draw_connected_samples(samples.iter().map(|sample| &sample.s), stroke, painter)
    }

    pub fn draw_trajectory(&self, stroke: Stroke, painter: &Painter) {
        self.draw_connected_samples(self.trajectory.iter(), stroke, painter);
    }

    fn draw_connected_samples<'a, Iter>(&self, samples: Iter, stroke: Stroke, painter: &Painter)
    where
        Iter: Iterator<Item = &'a Vec3>,
    {
        samples.map(|s| self.user_to_screen(*s)).reduce(|u0, u1| {
            // avoid drawing extremely short line segments:
            if (u0.x - u1.x).abs() > 2. || (u0.y - u1.y).abs() > 2. {
                painter.line_segment([u0, u1], stroke);
                u1
            } else {
                u0
            }
        });
    }

    pub fn draw_sample_dots(&self, samples: &[Sample], color: Color32, painter: &Painter) {
        samples
            .iter()
            .map(|sample| self.user_to_screen(sample.s))
            .fold(Pos2::new(f32::MAX, f32::MAX), |u0, u1| {
                if (u0.x - u1.x).abs() > 1. || (u0.y - u1.y).abs() > 1. {
                    painter.circle_filled(u1, 2.5, color);
                    u1
                } else {
                    u0
                }
            });
    }

    fn adjust_scale_and_center(&mut self, paint_area: &egui::Rect) {
        let scale = f32::min(paint_area.width(), paint_area.height()) / self.visible_units;
        self.scale = Vec3::new(scale, -scale, 1.);
        self.area_center = paint_area.center();
    }

    pub fn allocate_painter(&mut self, ui: &mut Ui, size: Vec2) -> (Response, Painter) {
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        self.interact(ui, &response);
        self.adjust_scale_and_center(&response.rect);
        (response, painter)
    }

    fn interact(&mut self, ui: &Ui, response: &Response) {
        if response.hovered() {
            let input = ui.input();
            if input.modifiers.command {
                let Vec2 { x: _, y: scroll_y } = input.pointer.delta();
                self.visible_units = clamp(self.visible_units * 1.01_f32.powf(scroll_y), 0.1..=20.);
            } else if input.pointer.button_down(PointerButton::Primary) {
                let mouse_delta = ui.input().pointer.delta();
                let screen_focus = self.user_to_screen(self.focus);
                self.focus = self.screen_to_user(screen_focus - mouse_delta);
            }
        }
    }

    pub fn on_hover_ui(&self, response: &Response, add_contents: impl FnOnce(&mut Ui, Vec3)) {
        //** this used to work with bevy_egui, before switching to eframe, but now
        //** the tooltip is placed _below_ the current paint area:
        //
        // response.clone().on_hover_ui(|ui| {
        //     if let Some(mouse_pos) = ui.input().pointer.tooltip_pos() {
        //         add_contents(ui, self.screen_to_user(mouse_pos));
        //     }
        // });
        if response.hovered() && response.ctx.input().pointer.has_pointer() {
            egui::popup::show_tooltip_at_pointer(
                &response.ctx,
                response.id.with("tooltip"),
                |ui| {
                    if let Some(mouse_pos) = ui.input().pointer.tooltip_pos() {
                        add_contents(ui, self.screen_to_user(mouse_pos));
                    }
                },
            );
        }
    }

    pub fn draw_line_segment(&self, start: Vec3, end: Vec3, stroke: Stroke, painter: &Painter) {
        painter.line_segment(
            [self.user_to_screen(start), self.user_to_screen(end)],
            stroke,
        )
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn draw_vector(&self, start: Vec3, vec: Vec3, stroke: Stroke, painter: &Painter) {
        let end = self.user_to_screen(start + vec);
        let start = self.user_to_screen(start);
        painter.line_segment([start, end], stroke);
        let direction = end - start;
        let direction_normalized =
            direction / (direction.x * direction.x + direction.y * direction.y).sqrt();
        let mut tail = [Pos2::new(0., -2.), Pos2::new(0., 2.)];
        // the vec![] macro does not work here...
        let mut tip = Vec::with_capacity(3);
        tip.push(Pos2::ZERO);
        tip.push(Pos2::new(-6., -2.));
        tip.push(Pos2::new(-6., 2.));
        rotate(&mut tail, direction_normalized);
        rotate(&mut tip, direction_normalized);
        move_to(&mut tail, start);
        move_to(&mut tip, end);
        painter.add(Shape::polygon(tip, stroke.color, stroke));
        painter.line_segment(tail, stroke)
    }

    pub fn draw_hline(&self, y: f32, stroke: Stroke, paint_area: &egui::Rect, painter: &Painter) {
        let transformed_y = self.user_to_screen(Vec3::new(0., y, 0.)).y;
        painter.line_segment(
            [
                Pos2::new(paint_area.left(), transformed_y),
                Pos2::new(paint_area.right(), transformed_y),
            ],
            stroke,
        );
    }

    pub fn draw_vline(&self, x: f32, stroke: Stroke, paint_area: &egui::Rect, painter: &Painter) {
        let transformed_x = self.user_to_screen(Vec3::new(x, 0., 0.)).x;
        painter.line_segment(
            [
                Pos2::new(transformed_x, paint_area.top()),
                Pos2::new(transformed_x, paint_area.bottom()),
            ],
            stroke,
        );
    }

    pub fn min(&self, paint_area: &egui::Rect) -> Vec3 {
        self.screen_to_user(Pos2::new(
            paint_area.min.x,
            paint_area.max.y, // user coords go from bottom to top
        ))
    }

    pub fn max(&self, paint_area: &egui::Rect) -> Vec3 {
        self.screen_to_user(Pos2::new(
            paint_area.max.x,
            paint_area.min.y, // user coords go from bottom to top
        ))
    }

    fn user_to_screen(&self, pos: Vec3) -> Pos2 {
        ((pos - self.focus) * self.scale).to_pos2() + self.area_center.to_vec2()
    }

    fn screen_to_user(&self, pos: Pos2) -> Vec3 {
        (pos - self.area_center.to_vec2()).to_vec3() / self.scale + self.focus
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

trait ToPos2 {
    fn to_pos2(&self) -> Pos2;
}

trait ToVec3 {
    fn to_vec3(&self) -> Vec3;
}

impl ToVec3 for Pos2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

impl ToVec3 for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

impl ToPos2 for Vec3 {
    fn to_pos2(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }
}
