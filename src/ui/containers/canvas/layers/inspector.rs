use super::{
    core::{
        derived_quantities::{Contribution, QuantityKind},
        Obj,
    },
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
        canvas.on_hover(response, |mouse_pos| {
            if let Some((ref_sample, calc_sample)) = integration.borrow().closest_sample(mouse_pos)
            {
                // *** reference sample:
                let ref_sample_dt = ref_sample.dt.into_inner();
                // delta s by velocity:
                canvas.draw_vector(
                    ref_sample.derived_position.as_position(),
                    ref_sample.derived_velocity.as_velocity() * ref_sample_dt,
                    settings.strokes.focussed_velocity,
                    painter,
                );
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    ref_sample.derived_position.as_position(),
                    0.5 * ref_sample.acceleration * ref_sample_dt * ref_sample_dt,
                    settings.strokes.focussed_acceleration,
                    painter,
                );

                // first, draw contributing translations...:
                for position_contribution in &calc_sample.derived_position.contributions {
                    if let Some(delta) = position_contribution.delta() {
                        canvas.draw_vector(
                            position_contribution.sampling_position(),
                            delta,
                            match position_contribution.base_quantity() {
                                QuantityKind::Velocity => settings.strokes.focussed_velocity,
                                QuantityKind::Acceleration => {
                                    settings.strokes.focussed_acceleration
                                }
                                QuantityKind::Position => {
                                    panic!("Trying to draw a position as a vector")
                                }
                            },
                            painter,
                        );
                    }
                }

                // ...then draw start position(s) on top, so they are visible...
                for position_contribution in &calc_sample.derived_position.contributions {
                    if position_contribution.delta().is_none() {
                        canvas.draw_sample_dot(
                            position_contribution.sampling_position(),
                            Color32::RED,
                            painter,
                        );
                    }
                }

                // ...finally, draw derived position:
                canvas.draw_sample_dot(
                    calc_sample.derived_position.as_position(),
                    Color32::GREEN,
                    painter,
                );

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
