use crate::{scenarios::Scenario, ui::UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(inspector.system());
    }
}

pub fn inspector(ui_state: ResMut<UIState>, scenarios: Query<&Scenario>) {
    if !ui_state.layerflags.inspector {
        return;
    }
    let canvas = &ui_state.canvas;
    for scenario in scenarios.iter() {
        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some(sample) = scenario.closest_sample(mouse_pos) {
                canvas.vector(
                    sample.s,
                    sample.v * scenario.step_duration(),
                    ui_state.strokes.focussed_velocity,
                );
                let a = scenario.acceleration().value_at(sample.s);
                canvas.vector(
                    sample.s,
                    a * scenario.step_duration(),
                    ui_state.strokes.focussed_acceleration,
                );

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    sample.n,
                    ui_state.format_f32(sample.t)
                ));
            }
        });
    }
}
