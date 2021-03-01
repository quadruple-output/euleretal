use crate::{canvas, scenario, UiState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render.system());
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn render(
    ui_state: Res<UiState>,
    scenarios: Query<(&scenario::Kind, &scenario::comp::Acceleration)>,
    mut canvases: Query<(&canvas::Kind, &mut canvas::State, &scenario::Entity)>, // always request canvases with 'mut'
) {
    if !ui_state.layerflags.acceleration_field {
        return;
    }

    for (_, canvas, scenario_id) in canvases.iter_mut() {
        let (_, acceleration) = scenarios.get(scenario_id.0).unwrap();

        let min = canvas.min();
        let max = canvas.max();
        for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
                let pos = Vec3::new(x as f32, y as f32, 0.);
                let a = acceleration.value_at(pos);
                canvas.draw_vector(pos, a, ui_state.strokes.acceleration)
            }
        }

        canvas.on_hover_ui(|ui, mouse_pos| {
            let a = acceleration.value_at(mouse_pos);
            ui.label("Field");
            ui.separator();
            ui.label(format!("|a| = {}", ui_state.format_f32(a.length())));
            canvas.draw_vector(mouse_pos, a, ui_state.strokes.acceleration)
        })
    }
}
