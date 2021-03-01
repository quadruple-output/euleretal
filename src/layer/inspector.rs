use crate::{canvas, integration, UiState};
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
    integrations: Query<(
        &integration::Kind,
        &integration::comp::State,
        &integration::comp::CanvasId,
    )>,
    mut canvases: Query<(&canvas::Kind, &mut canvas::comp::State)>,
) {
    if !ui_state.layerflags.inspector {
        return;
    }
    for (_, integration, canvas_id) in integrations.iter() {
        let (_, canvas) = canvases.get_mut(canvas_id.0).unwrap();

        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some((ref_sample, calc_sample)) = integration.closest_sample(mouse_pos) {
                // *** reference sample:
                let ref_sample_dt = ref_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    ref_sample.s,
                    ref_sample.v * ref_sample_dt,
                    ui_state.strokes.focussed_velocity,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    ref_sample.s,
                    0.5 * ref_sample.a * ref_sample_dt * ref_sample_dt,
                    ui_state.strokes.focussed_acceleration,
                );
                // *** calculated sample:
                let calc_sample_dt = calc_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    calc_sample.s,
                    calc_sample.v * calc_sample_dt,
                    ui_state.strokes.focussed_velocity,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
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
