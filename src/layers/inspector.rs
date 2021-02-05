use crate::{Canvas, CanvasId, Scenario, ScenarioId, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(inspector.system());
    }
}

pub fn inspector(
    ui_state: ResMut<UIState>,
    integration_views: Query<(&ScenarioId, &CanvasId)>,
    mut scenarios: Query<&mut Scenario>,
    mut canvases: Query<&mut Canvas>,
) {
    if !ui_state.layerflags.inspector {
        return;
    }
    for (scenario_id, canvas_id) in integration_views.iter() {
        let scenario = scenarios.get_mut(scenario_id.0).unwrap();
        let canvas = canvases.get_mut(canvas_id.0).unwrap();
        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some(sample) = scenario.closest_sample(mouse_pos) {
                canvas.vector(
                    sample.s,
                    sample.v * sample.dt,
                    ui_state.strokes.focussed_velocity,
                );
                let a = scenario.acceleration().value_at(sample.s);
                canvas.vector(
                    sample.s,
                    a * sample.dt,
                    ui_state.strokes.focussed_acceleration,
                );

                ui.label("Inspector");
                ui.separator();
                ui.label(match sample.n {
                    Some(n) => format!("#{}: t = {}", n, ui_state.format_f32(sample.t)),
                    None => format!("t= {}", ui_state.format_f32(sample.t)),
                });
            }
        });
    }
}
