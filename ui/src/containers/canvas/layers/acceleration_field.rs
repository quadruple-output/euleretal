use super::{core::Position, entities::CanvasPainter, import::Vec3, misc};

pub fn render(settings: &misc::Settings, canvas: &CanvasPainter) {
    #![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

    let scenario_obj = canvas.scenario(); // need temp var to extend lifetime
    let scenario = scenario_obj.borrow();
    let acceleration = &scenario.acceleration;

    let min = canvas.rect_min();
    let max = canvas.rect_max();
    for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
            let pos = Position::new(x as f32, y as f32, 0.);
            let a = acceleration.value_at(pos);
            canvas.draw_vector(pos, a, settings.strokes.acceleration);
        }
    }

    canvas.on_hover_ui(|ui, mouse_pos| {
        let a = acceleration.value_at(mouse_pos.into());
        ui.label("Field");
        ui.separator();
        ui.label(format!(
            "|a| = {}",
            settings.format_f32(Vec3::from(a).norm())
        ));
        canvas.draw_vector(mouse_pos, a, settings.strokes.acceleration);
    });
}
