use crate::{Canvas, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(display_coordinates.system());
    }
}

fn display_coordinates(
    // UIState must be requested as Mut, or else it panics when other systems use it in parallel
    ui_state: ResMut<UIState>,
    mut canvases: Query<&mut Canvas>,
) {
    if !ui_state.layerflags.coordinates {
        return;
    }
    for canvas in canvases.iter_mut() {
        canvas.hline(0., ui_state.strokes.coordinates);
        canvas.vline(0., ui_state.strokes.coordinates);
        let min = canvas.min();
        let max = canvas.max();
        for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            canvas.line_segment(
                Vec3::new(step as f32, -0.05, 0.),
                Vec3::new(step as f32, 0.05, 0.),
                ui_state.strokes.coordinates,
            );
        }
        for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
            canvas.line_segment(
                Vec3::new(-0.05, step as f32, 1.),
                Vec3::new(0.05, step as f32, 1.),
                ui_state.strokes.coordinates,
            );
        }

        canvas.on_hover_ui(|ui, pos| {
            ui.label(format!("x = {}", ui_state.format_f32(pos.x)));
            ui.label(format!("y = {}", ui_state.format_f32(pos.y)));
        });
    }
}
