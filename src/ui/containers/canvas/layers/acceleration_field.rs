use super::{core::Obj, entities::Canvas, import::Vec3, misc, ui_import::egui};

pub fn render(
    settings: &misc::Settings,
    canvas: &Obj<Canvas>,
    response: &egui::Response,
    painter: &egui::Painter,
) {
    let canvas = canvas.borrow();
    let scenario = canvas.scenario().borrow();
    let acceleration = &scenario.acceleration;

    let min = canvas.min(&response.rect);
    let max = canvas.max(&response.rect);
    for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
            let pos = Vec3::new(x as f32, y as f32, 0.);
            let a = acceleration.value_at(pos);
            canvas.draw_vector(pos, a, settings.strokes.acceleration, painter)
        }
    }

    canvas.on_hover_ui(response, |ui, mouse_pos| {
        let a = acceleration.value_at(mouse_pos);
        ui.label("Field");
        ui.separator();
        ui.label(format!("|a| = {}", settings.format_f32(a.length())));
        canvas.draw_vector(mouse_pos, a, settings.strokes.acceleration, painter)
    });
}
