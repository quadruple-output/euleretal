use crate::{scenarios::Scenario, ui::UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(selector.system());
    }
}

pub fn selector(ui_state: ResMut<UIState>, scenarios: Query<&Scenario>) {
    let canvas = &ui_state.canvas;
    for scenario in scenarios.iter() {
        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some(sample) = scenario.closest_sample(mouse_pos) {
                ui.label(format!("t: {:.3}", sample.t));
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
            }
        });
    }
}
