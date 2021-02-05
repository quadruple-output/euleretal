use crate::{Canvas, CanvasId, Scenario, ScenarioId, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_acceleration.system());
    }
}

pub fn render_acceleration(
    // UIState must be requested as Mut, or else it panics when other systems use it in parallel
    ui_state: ResMut<UIState>,
    integration_views: Query<(&ScenarioId, &CanvasId)>,
    mut scenarios: Query<&mut Scenario>,
    mut canvases: Query<&mut Canvas>,
) {
    if !ui_state.layerflags.acceleration_field {
        return;
    }

    for (scenario_id, canvas_id) in integration_views.iter() {
        let scenario = scenarios.get_mut(scenario_id.0).unwrap();
        let canvas = canvases.get_mut(canvas_id.0).unwrap();

        let min = canvas.min();
        let max = canvas.max();
        for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
                let pos = Vec3::new(x as f32, y as f32, 0.);
                let a = scenario.acceleration().value_at(pos);
                canvas.vector(pos, a, ui_state.strokes.acceleration)
            }
        }

        canvas.on_hover_ui(|ui, mouse_pos| {
            let a = scenario.acceleration().value_at(mouse_pos);
            ui.label("Field");
            ui.separator();
            ui.label(format!("|a| = {}", ui_state.format_f32(a.length())));
            canvas.vector(mouse_pos, a, ui_state.strokes.acceleration)
        })
    }
}
