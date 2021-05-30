use super::{core::Obj, entities::Canvas, misc::Settings, ui_import::egui};

pub fn render(
    settings: &Settings,
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
                    settings.strokes.focussed_velocity,
                    painter,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    ref_sample.s,
                    0.5 * ref_sample.a * ref_sample_dt * ref_sample_dt,
                    settings.strokes.focussed_acceleration,
                    painter,
                );
                // *** calculated sample:
                if calc_sample.calibration_points.is_empty() {
                    let calc_sample_dt = calc_sample.dt.into_inner();
                    // delta s by velocity:
                    canvas.draw_vector(
                        calc_sample.s,
                        calc_sample.v * calc_sample_dt,
                        settings.strokes.focussed_velocity,
                        painter,
                    );
                    // delta s by acceleration at sample point:
                    canvas.draw_vector(
                        calc_sample.s,
                        0.5 * calc_sample.a * calc_sample_dt * calc_sample_dt,
                        settings.strokes.focussed_acceleration,
                        painter,
                    );
                } else {
                    // calibration Points:
                    for point in calc_sample.calibration_points {
                        if let Some(eff_velocity) = point.eff_velocity {
                            canvas.draw_vector(
                                point.position,
                                eff_velocity,
                                settings.strokes.focussed_velocity,
                                painter,
                            );
                        }
                        if let Some(eff_acceleration) = point.eff_acceleration {
                            canvas.draw_vector(
                                point.position,
                                eff_acceleration,
                                settings.strokes.focussed_acceleration,
                                painter,
                            );
                        }
                    }
                }

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    calc_sample.n,
                    settings.format_f32(calc_sample.t.into())
                ));
                ui.label(format!(
                    "ds = {}",
                    settings.format_f32((calc_sample.s - ref_sample.s).length())
                ));
                ui.label(format!(
                    "dv = {}",
                    settings.format_f32((calc_sample.v - ref_sample.v).length())
                ));
                ui.label(format!(
                    "da = {}",
                    settings.format_f32((calc_sample.a - ref_sample.a).length())
                ));
            }
        });
    })
}
