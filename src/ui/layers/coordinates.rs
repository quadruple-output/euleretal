use crate::prelude::*;

pub fn render(world: &mut World, state: &ControlState) {
    if !state.layerflags.coordinates {
        return;
    }
    for canvas in world.query::<&canvas::comp::State>() {
        canvas.draw_hline(0., state.strokes.coordinates);
        canvas.draw_vline(0., state.strokes.coordinates);
        let min = canvas.min();
        let max = canvas.max();
        for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            canvas.draw_line_segment(
                Vec3::new(step as f32, -0.05, 0.),
                Vec3::new(step as f32, 0.05, 0.),
                state.strokes.coordinates,
            );
        }
        for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
            canvas.draw_line_segment(
                Vec3::new(-0.05, step as f32, 1.),
                Vec3::new(0.05, step as f32, 1.),
                state.strokes.coordinates,
            );
        }

        // canvas.on_hover_ui(|ui, pos| {
        //     ui.label(format!("x = {}", ui_state.format_f32(pos.x)));
        //     ui.label(format!("y = {}", ui_state.format_f32(pos.y)));
        // });
    }
}
