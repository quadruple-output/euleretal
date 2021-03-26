use crate::prelude::*;

pub fn render(
    world: &World,
    state: &ControlState,
    canvas_id: bevy_ecs::Entity,
    paint_area: &egui::Rect,
    painter: &egui::Painter,
) {
    let canvas = world.get::<canvas::comp::State>(canvas_id).unwrap();
    canvas.draw_hline(0., state.strokes.coordinates, paint_area, painter);
    canvas.draw_vline(0., state.strokes.coordinates, paint_area, painter);
    let min = canvas.min(paint_area);
    let max = canvas.max(paint_area);
    for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        canvas.draw_line_segment(
            Vec3::new(step as f32, -0.05, 0.),
            Vec3::new(step as f32, 0.05, 0.),
            state.strokes.coordinates,
            painter,
        );
    }
    for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
        canvas.draw_line_segment(
            Vec3::new(-0.05, step as f32, 1.),
            Vec3::new(0.05, step as f32, 1.),
            state.strokes.coordinates,
            painter,
        );
    }

    // canvas.on_hover_ui(|ui, pos| {
    //     ui.label(format!("x = {}", ui_state.format_f32(pos.x)));
    //     ui.label(format!("y = {}", ui_state.format_f32(pos.y)));
    // });
}
