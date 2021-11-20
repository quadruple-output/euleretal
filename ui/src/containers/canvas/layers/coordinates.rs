use super::{entities::CanvasPainter, import::Point3, misc::settings};

pub fn render(canvas: &CanvasPainter, strokes: &settings::Strokes) {
    #![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    canvas.draw_hline(0., strokes.coordinates);
    canvas.draw_vline(0., strokes.coordinates);
    let min = canvas.rect_min();
    let max = canvas.rect_max();
    for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        canvas.draw_line_segment(
            Point3::new(step as f32, -0.05, 0.),
            Point3::new(step as f32, 0.05, 0.),
            strokes.coordinates,
        );
    }
    for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
        canvas.draw_line_segment(
            Point3::new(-0.05, step as f32, 1.),
            Point3::new(0.05, step as f32, 1.),
            strokes.coordinates,
        );
    }

    // canvas.on_hover_ui(|ui, pos| {
    //     ui.label(format!("x = {}", ui_state.format_f32(pos.x)));
    //     ui.label(format!("y = {}", ui_state.format_f32(pos.y)));
    // });
}
