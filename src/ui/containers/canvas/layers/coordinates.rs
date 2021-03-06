use super::{core::Position, entities::CanvasPainter, misc::settings};

pub fn render(strokes: &settings::Strokes, canvas: &CanvasPainter) {
    canvas.draw_hline(0., strokes.coordinates);
    canvas.draw_vline(0., strokes.coordinates);
    let min = canvas.rect_min();
    let max = canvas.rect_max();
    for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        canvas.draw_line_segment(
            Position::new(step as f32, -0.05, 0.),
            Position::new(step as f32, 0.05, 0.),
            strokes.coordinates,
        );
    }
    for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
        canvas.draw_line_segment(
            Position::new(-0.05, step as f32, 1.),
            Position::new(0.05, step as f32, 1.),
            strokes.coordinates,
        );
    }

    // canvas.on_hover_ui(|ui, pos| {
    //     ui.label(format!("x = {}", ui_state.format_f32(pos.x)));
    //     ui.label(format!("y = {}", ui_state.format_f32(pos.y)));
    // });
}
