use super::{
    core::Obj,
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
                    ref_sample.last_s(),
                    ref_sample.last_v() * ref_sample_dt,
                    settings.strokes.focussed_velocity,
                    painter,
                );
                /*
                // delta s by acceleration at sample point:
                canvas.draw_vector(
                    ref_sample.derived_position.as_position(),
                    0.5 * ref_sample.acceleration * ref_sample_dt * ref_sample_dt,
                    settings.strokes.focussed_acceleration,
                    painter,
                );
                */

                // first, draw contributing translations...:
                for (sampling_position, velocity) in calc_sample.velocities_iter() {
                    canvas.draw_vector(
                        sampling_position,
                        velocity, // todo: this is the absolute velocity. Need effective contribution.
                        settings.strokes.focussed_velocity,
                        painter,
                    );
                }

                // first, draw contributing translations...:
                for (sampling_position, acceleration) in calc_sample.accelerations_iter() {
                    canvas.draw_vector(
                        sampling_position,
                        acceleration, // todo: this is the absolute acceleration. Need effective contribution.
                        settings.strokes.focussed_acceleration,
                        painter,
                    );
                }

                // ...then draw start position(s) on top, so they are visible...
                let last_s = calc_sample.last_s();
                for position in calc_sample.positions_iter() {
                    if position != last_s {
                        canvas.draw_sample_dot(position, Color32::RED, painter);
                    }
                }

                // ...finally, draw derived position:
                canvas.draw_sample_dot(last_s, Color32::GREEN, painter);

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
