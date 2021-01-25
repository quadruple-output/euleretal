use bevy::math::Vec3;
use egui::{Painter, Pos2, Response, Stroke, Ui, Vec2};

pub struct Canvas {
    response: Response,
    painter: Painter,
    translation: Vec3,
    scale: Vec3,
}

impl Canvas {
    pub fn new(allocated_painter: (Response, Painter), visible_units: f32) -> Self {
        let (response, painter) = allocated_painter;
        let area = response.rect;
        let translation = area.center().to_vec3();
        let scale = f32::min(area.width(), area.height()) / visible_units;
        let scale = Vec3::new(scale, -scale, 1.);
        Self {
            response,
            painter,
            translation,
            scale,
        }
    }

    pub fn on_hover_ui(&self, f: impl FnOnce(&mut Ui, Vec3)) {
        self.response.clone().on_hover_ui(|ui| {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                f(ui, self.screen_to_user(mouse_pos));
            }
        });
    }

    pub fn line_segment(&self, start: Vec3, end: Vec3, stroke: Stroke) {
        self.painter.line_segment(
            [self.user_to_screen(start), self.user_to_screen(end)],
            stroke,
        )
    }

    pub fn hline(&self, y: f32, stroke: Stroke) {
        let area = self.response.rect;
        let transformed_y = self.translation.y + self.scale.y * y;
        self.painter.line_segment(
            [
                Pos2::new(area.left(), transformed_y),
                Pos2::new(area.right(), transformed_y),
            ],
            stroke,
        );
    }

    pub fn vline(&self, x: f32, stroke: Stroke) {
        let area = self.response.rect;
        let transformed_x = self.translation.x + self.scale.x * x;
        self.painter.line_segment(
            [
                Pos2::new(transformed_x, area.top()),
                Pos2::new(transformed_x, area.bottom()),
            ],
            stroke,
        );
    }

    pub fn min(&self) -> Vec3 {
        self.screen_to_user(Pos2::new(
            self.response.rect.min.x,
            self.response.rect.max.y, // user coords go from bottom to top
        ))
    }

    pub fn max(&self) -> Vec3 {
        self.screen_to_user(Pos2::new(
            self.response.rect.max.x,
            self.response.rect.min.y, // user coords go from bottom to top
        ))
    }

    fn user_to_screen(&self, pos: Vec3) -> Pos2 {
        (pos * self.scale + self.translation).to_pos2()
    }

    fn screen_to_user(&self, pos: Pos2) -> Vec3 {
        (pos.to_vec3() - self.translation) / self.scale
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
