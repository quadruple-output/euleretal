use super::{
    core::{Obj, PhysicalQuantityKind},
    entities::Canvas,
    misc::Settings,
    ui_import::{egui, Color32, Stroke},
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
                let focus_on_velocity = response.ctx.input().modifiers.alt;

                // *** reference sample:
                // mark current reference sample with color:
                if focus_on_velocity {
                    // delta s by velocity:
                    canvas.draw_vector(
                        ref_sample.last_s(),
                        ref_sample.last_v() * ref_sample.dt,
                        Stroke::new(1., Color32::GREEN),
                        // settings.strokes.focussed_velocity,
                        painter,
                    );
                } else {
                    canvas.draw_sample_dot(ref_sample.last_s(), Color32::GREEN, painter);
                }

                // *** calculated sample:
                if focus_on_velocity {
                    let last_velocity = calc_sample.last_computed_velocity();
                    for contribution in last_velocity.contributions_iter() {
                        canvas.draw_vector(
                            contribution.sampling_position(),
                            contribution.vector() * calc_sample.dt,
                            match contribution.kind() {
                                PhysicalQuantityKind::Position => panic!(),
                                PhysicalQuantityKind::Velocity => {
                                    Stroke::new(1., Color32::RED)
                                    //settings.strokes.focussed_velocity
                                }
                                PhysicalQuantityKind::Acceleration => {
                                    settings.strokes.focussed_acceleration
                                }
                            },
                            painter,
                        );
                    }
                    canvas.draw_vector(
                        last_velocity.sampling_position(),
                        last_velocity.v() * calc_sample.dt,
                        Stroke::new(1., Color32::GREEN),
                        painter,
                    );
                } else {
                    let last_position = calc_sample.last_computed_position();

                    // draw vectors first...
                    for contribution in last_position.contributions_iter() {
                        match contribution.kind() {
                            PhysicalQuantityKind::Position => {}
                            PhysicalQuantityKind::Velocity => {
                                canvas.draw_vector(
                                    contribution.sampling_position(),
                                    contribution.vector().unwrap(),
                                    settings.strokes.focussed_velocity,
                                    painter,
                                );
                            }
                            PhysicalQuantityKind::Acceleration => {
                                canvas.draw_vector(
                                    contribution.sampling_position(),
                                    contribution.vector().unwrap(),
                                    settings.strokes.focussed_acceleration,
                                    painter,
                                );
                            }
                        }
                    }

                    // ... then draw points on top:
                    for contribution in last_position.contributions_iter() {
                        match contribution.kind() {
                            PhysicalQuantityKind::Position => {
                                canvas.draw_sample_dot(
                                    contribution.sampling_position(),
                                    Color32::RED,
                                    painter,
                                );
                            }
                            PhysicalQuantityKind::Velocity | PhysicalQuantityKind::Acceleration => {
                            }
                        }
                    }

                    // finally draw the calculated sample position:
                    canvas.draw_sample_dot(last_position.s(), Color32::GREEN, painter);
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
    });
}
