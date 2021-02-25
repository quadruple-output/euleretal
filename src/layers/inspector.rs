use crate::{Canvas, Integration, UiState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(render.system());
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn render(
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
                // *** reference sample:
                let ref_sample_dt = ref_sample.dt.into_inner();
                // delta s by velocity:
                canvas.vector(
                    ref_sample.s,
                    ref_sample.v * ref_sample_dt,
                    ui_state.strokes.focussed_velocity,
                );
                // delta s by acceleration at sample point:
                canvas.vector(
                    ref_sample.s,
                    0.5 * ref_sample.a * ref_sample_dt * ref_sample_dt,
                    ui_state.strokes.focussed_acceleration,
                );
                // *** calculated sample:
                let calc_sample_dt = calc_sample.dt.into_inner();
                // delta s by velocity:
                canvas.vector(
                    calc_sample.s,
                    calc_sample.v * calc_sample_dt,
                    ui_state.strokes.focussed_velocity,
                );
                // delta s by acceleration at sample point:
                canvas.vector(
                    calc_sample.s,
                    0.5 * calc_sample.a * calc_sample_dt * calc_sample_dt,
                    ui_state.strokes.focussed_acceleration,
                );

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    calc_sample.n,
                    ui_state.format_f32(calc_sample.t.into())
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
