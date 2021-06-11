use super::{
    core::{derived_quantities::PositionFragment, Obj, Velocity},
    entities::Canvas,
    misc::Settings,
    ui_import::{egui, Color32},
};

pub fn render(
    settings: &Settings,
    canvas: &Obj<Canvas>,
    response: &egui::Response,
    painter: &egui::Painter,
) {
    let canvas = canvas.borrow();
    canvas.integrations().for_each(|integration| {
        canvas.on_hover_ui(response, |_ui, mouse_pos| {
            if let Some((ref_sample, calc_sample)) = integration.borrow().closest_sample(mouse_pos)
            {
                // *** reference sample:
                let ref_sample_dt = ref_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    (&ref_sample.derived_position).into(),
                    Velocity::from(&ref_sample.derived_velocity) * ref_sample_dt,
                    settings.strokes.focussed_velocity,
                    painter,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    (&ref_sample.derived_position).into(),
                    0.5 * ref_sample.acceleration * ref_sample_dt * ref_sample_dt,
                    settings.strokes.focussed_acceleration,
                    painter,
                );

                for position_contribution in &calc_sample.derived_position.contributions {
                    match &position_contribution.quantity {
                        PositionFragment::Position {} => {
                            canvas.draw_sample_dot(
                                (&position_contribution.sampling_position).into(),
                                Color32::LIGHT_GRAY,
                                painter,
                            );
                        }
                        PositionFragment::VelocityDt {
                            factor: _,
                            v: _,
                            dt: _,
                            dt_fraction: _,
                        } => canvas.draw_vector(
                            (&position_contribution.sampling_position).into(),
                            position_contribution.eff_position(),
                            settings.strokes.focussed_velocity,
                            painter,
                        ),
                        PositionFragment::AccelerationDtDt {
                            factor: _,
                            a: _,
                            dt: _,
                            dt_fraction: _,
                        } => canvas.draw_vector(
                            (&position_contribution.sampling_position).into(),
                            position_contribution.eff_position(),
                            settings.strokes.focussed_acceleration,
                            painter,
                        ),
                    };
                }
                // ui.label("Inspector");
                // ui.separator();
                // ui.label(format!(
                //     "#{}: t = {}",
                //     calc_sample.n,
                //     settings.format_f32(calc_sample.t.into())
                // ));
                // ui.label(format!(
                //     "ds = {}",
                //     settings.format_f32((calc_sample.s - ref_sample.s).length())
                // ));
                // ui.label(format!(
                //     "dv = {}",
                //     settings.format_f32((calc_sample.v - ref_sample.v).length())
                // ));
                // ui.label(format!(
                //     "da = {}",
                //     settings.format_f32((calc_sample.a - ref_sample.a).length())
                // ));
            }
        });
    })
}
