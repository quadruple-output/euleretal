use crate::{Canvas, Integration, UiState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(inspector.system());
    }
}

pub fn inspector(
    ui_state: ResMut<UiState>,
    integrations: Query<&Integration>,
    mut canvases: Query<&mut Canvas>,
) {
    if !ui_state.layerflags.inspector {
        return;
    }
    for integration in integrations.iter() {
        let canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();

        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some((ref_sample, calc_sample)) = integration.closest_sample(mouse_pos) {
                canvas.vector(
                    ref_sample.s,
                    ref_sample.v * ref_sample.dt,
                    ui_state.strokes.focussed_velocity,
                );
                canvas.vector(
                    ref_sample.s,
                    ref_sample.a * ref_sample.dt,
                    ui_state.strokes.focussed_acceleration,
                );
                canvas.vector(
                    calc_sample.s,
                    calc_sample.v * calc_sample.dt,
                    ui_state.strokes.focussed_velocity,
                );
                canvas.vector(
                    calc_sample.s,
                    calc_sample.a * calc_sample.dt,
                    ui_state.strokes.focussed_acceleration,
                );

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    calc_sample.n,
                    ui_state.format_f32(calc_sample.t)
                ));
                ui.label(format!(
                    "ds = {}",
                    ui_state.format_f32((calc_sample.s - ref_sample.s).length())
                ));
                ui.label(format!(
                    "dv = {}",
                    ui_state.format_f32((calc_sample.v - ref_sample.v).length())
                ));
                ui.label(format!(
                    "da = {}",
                    ui_state.format_f32((calc_sample.a - ref_sample.a).length())
                ));
            }
        });
    }
}
