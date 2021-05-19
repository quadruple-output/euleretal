use super::super::{Canvas, ControlState};
use crate::prelude::*;

pub fn render(
    state: &ControlState,
    canvas: &Obj<Canvas>,
    response: &egui::Response,
    painter: &egui::Painter,
) {
    let canvas = canvas.borrow();
    canvas.integrations().for_each(|integration| {
        canvas.on_hover_ui(response, |ui, mouse_pos| {
            if let Some((ref_sample, calc_sample)) = integration.borrow().closest_sample(mouse_pos)
            {
                // *** reference sample:
                let ref_sample_dt = ref_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    ref_sample.s,
                    ref_sample.v * ref_sample_dt,
                    state.strokes.focussed_velocity,
                    painter,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    ref_sample.s,
                    0.5 * ref_sample.a * ref_sample_dt * ref_sample_dt,
                    state.strokes.focussed_acceleration,
                    painter,
                );
                // *** calculated sample:
                let calc_sample_dt = calc_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    calc_sample.s,
                    calc_sample.v * calc_sample_dt,
                    state.strokes.focussed_velocity,
                    painter,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    calc_sample.s,
                    0.5 * calc_sample.a * calc_sample_dt * calc_sample_dt,
                    state.strokes.focussed_acceleration,
                    painter,
                );

                // calibration Points:
                for point in calc_sample.calibration_points {
                    canvas.draw_vector(
                        point.position,
                        0.5 * point.acceleration * calc_sample_dt * calc_sample_dt,
                        state.strokes.focussed_acceleration,
                        painter,
                    );
                }

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    calc_sample.n,
                    state.format_f32(calc_sample.t.into())
                ));
                ui.label(format!(
                    "ds = {}",
                    state.format_f32((calc_sample.s - ref_sample.s).length())
                ));
                ui.label(format!(
                    "dv = {}",
                    state.format_f32((calc_sample.v - ref_sample.v).length())
                ));
                ui.label(format!(
                    "da = {}",
                    state.format_f32((calc_sample.a - ref_sample.a).length())
                ));
            }
        });
    })
}
