use super::super::{Canvas, ControlState};
use crate::prelude::*;

pub fn render(
    state: &ControlState,
    canvas: &Obj<Canvas>,
    response: &egui::Response,
    painter: &egui::Painter,
) {
    if !state.layerflags.acceleration_field {
        return;
    }

    let canvas = canvas.borrow();
    let scenario = canvas.scenario().borrow();
    let acceleration = &scenario.acceleration;

    let min = canvas.min(&response.rect);
    let max = canvas.max(&response.rect);
    for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
            let pos = Vec3::new(x as f32, y as f32, 0.);
            let a = acceleration.value_at(pos);
            canvas.draw_vector(pos, a, state.strokes.acceleration, painter)
        }
    }

    canvas.on_hover_ui(response, |ui, mouse_pos| {
        let a = acceleration.value_at(mouse_pos);
        ui.label("Field");
        ui.separator();
        ui.label(format!("|a| = {}", state.format_f32(a.length())));
        canvas.draw_vector(mouse_pos, a, state.strokes.acceleration, painter)
    });
}
