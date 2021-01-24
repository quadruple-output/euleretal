use egui::{Painter, Pos2, Response, Stroke, Vec2};

pub struct Canvas {
    response: Response,
    painter: Painter,
    translation: Vec2,
    scale: Vec2,
}

impl Canvas {
    pub fn new(allocated_painter: (Response, Painter), visible_units: f32) -> Self {
        let (response, painter) = allocated_painter;
        let area = response.rect;
        let translation = area.center().to_vec2();
        let scale = f32::min(area.width(), area.height()) / visible_units;
        let scale = Vec2::new(scale, -scale);
        let response = response.on_hover_ui(move |ui| {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                let rel_pos = mouse_pos - translation;
                ui.label(format!("X: {:+.4}", rel_pos.x / scale.x));
                ui.label(format!("Y: {:+.4}", rel_pos.y / scale.y));
            }
        });
        Self {
            response,
            painter,
            translation,
            scale,
        }
    }

    pub fn line_segment(&self, start: Pos2, end: Pos2, stroke: Stroke) {
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

    pub fn min(&self) -> Pos2 {
        self.screen_to_user(Pos2::new(
            self.response.rect.min.x,
            self.response.rect.max.y, // user coords go from bottom to top
        ))
    }

    pub fn max(&self) -> Pos2 {
        self.screen_to_user(Pos2::new(
            self.response.rect.max.x,
            self.response.rect.min.y, // user coords go from bottom to top
        ))
    }

    fn user_to_screen(&self, pos: Pos2) -> Pos2 {
        Pos2::new(pos.x * self.scale.x, pos.y * self.scale.y) + self.translation
    }

    fn screen_to_user(&self, pos: Pos2) -> Pos2 {
        let pos_tl = pos - self.translation;
        Pos2::new(pos_tl.x / self.scale.x, pos_tl.y / self.scale.y)
    }
}
