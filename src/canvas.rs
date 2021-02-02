use bevy::math::Vec3;
use egui::{clamp, Color32, Painter, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2};

pub struct Canvas {
    allocated_painter: Option<(Response, Painter)>,
    visible_units: f32,
    focus: Vec3,
    area_center: Pos2,
    scale: Vec3,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            allocated_painter: None,
            visible_units: 2., // one unit in each direction
            focus: Vec3::zero(),
            area_center: Default::default(),
            scale: Default::default(),
        }
    }
}

impl Canvas {
    pub fn set_focus(&mut self, focus: Vec3) -> &mut Self {
        self.focus = focus;
        self
    }

    pub fn set_visible_units(&mut self, units: f32) -> &mut Self {
        self.visible_units = units;
        self.adjust_scale_and_center();
        self
    }

    fn adjust_scale_and_center(&mut self) {
        if let Some((ref response, _)) = self.allocated_painter {
            let area = response.rect;
            let scale = f32::min(area.width(), area.height()) / self.visible_units;
            self.scale = Vec3::new(scale, -scale, 1.);
            self.area_center = area.center();
        }
    }

    pub fn allocate_painter(&mut self, ui: &mut Ui, size: Vec2) {
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        self.interact(ui, &response);
        self.allocated_painter = Some((response, painter));
        self.adjust_scale_and_center();
    }

    fn interact(&mut self, ui: &Ui, response: &Response) {
        if response.hovered {
            let input = ui.input();
            if input.modifiers.command {
                let Vec2 { x: _, y: scroll_y } = input.mouse.delta;
                self.visible_units = clamp(self.visible_units * 1.01f32.powf(scroll_y), 0.1..=20.);
            } else if input.mouse.down {
                let mouse_delta = ui.input().mouse.delta;
                let screen_focus = self.user_to_screen(self.focus);
                self.focus = self.screen_to_user(screen_focus - mouse_delta);
            }
        }
    }

    pub fn on_hover_ui(&self, add_contents: impl FnOnce(&mut Ui, Vec3)) {
        if let Some((ref response, _)) = self.allocated_painter {
            response.clone().on_hover_ui(|ui| {
                if let Some(mouse_pos) = ui.input().mouse.pos {
                    add_contents(ui, self.screen_to_user(mouse_pos));
                }
            });
        }
    }

    pub fn line_segment(&self, start: Vec3, end: Vec3, stroke: Stroke) {
        if let Some((_, ref painter)) = self.allocated_painter {
            painter.line_segment(
                [self.user_to_screen(start), self.user_to_screen(end)],
                stroke,
            )
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn vector(&self, start: Vec3, vec: Vec3, stroke: Stroke) {
        if let Some((_, ref painter)) = self.allocated_painter {
            let end = self.user_to_screen(start + vec);
            let start = self.user_to_screen(start);
            painter.line_segment([start, end], stroke);
            let direction = end - start;
            let direction_normalized =
                direction / (direction.x * direction.x + direction.y * direction.y).sqrt();
            let mut tail = [Pos2::new(0., -2.), Pos2::new(0., 2.)];
            // the vec![] macro does not work here...
            let mut tip = Vec::with_capacity(3);
            tip.push(Pos2::zero());
            tip.push(Pos2::new(-6., -2.));
            tip.push(Pos2::new(-6., 2.));
            rotate(&mut tail, direction_normalized);
            rotate(&mut tip, direction_normalized);
            move_to(&mut tail, start);
            move_to(&mut tip, end);
            painter.add(Shape::polygon(tip, stroke.color, stroke));
            painter.line_segment(tail, stroke)
        }
    }

    pub fn dot(&self, pos: Vec3, color: Color32) {
        if let Some((_, ref painter)) = self.allocated_painter {
            painter.circle_filled(self.user_to_screen(pos), 2.5, color);
        }
    }

    pub fn hline(&self, y: f32, stroke: Stroke) {
        if let Some((ref response, ref painter)) = self.allocated_painter {
            let area = response.rect;
            let transformed_y = self.user_to_screen(Vec3::new(0., y, 0.)).y;
            painter.line_segment(
                [
                    Pos2::new(area.left(), transformed_y),
                    Pos2::new(area.right(), transformed_y),
                ],
                stroke,
            );
        }
    }

    pub fn vline(&self, x: f32, stroke: Stroke) {
        if let Some((ref response, ref painter)) = self.allocated_painter {
            let area = response.rect;
            let transformed_x = self.user_to_screen(Vec3::new(x, 0., 0.)).x;
            painter.line_segment(
                [
                    Pos2::new(transformed_x, area.top()),
                    Pos2::new(transformed_x, area.bottom()),
                ],
                stroke,
            );
        }
    }

    pub fn min(&self) -> Vec3 {
        let (ref response, _) = self.allocated_painter.as_ref().unwrap();
        self.screen_to_user(Pos2::new(
            response.rect.min.x,
            response.rect.max.y, // user coords go from bottom to top
        ))
    }

    pub fn max(&self) -> Vec3 {
        let (ref response, _) = self.allocated_painter.as_ref().unwrap();
        self.screen_to_user(Pos2::new(
            response.rect.max.x,
            response.rect.min.y, // user coords go from bottom to top
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
