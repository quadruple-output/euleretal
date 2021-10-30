use super::{
    core::{integration_step::computed, Contribution, Duration, PhysicalQuantityKind, Step},
    entities::CanvasPainter,
    misc::{Settings, StrokeExt},
};

pub fn render(settings: &Settings, canvas: &CanvasPainter) {
    let mut pointer_position = None;
    canvas.on_hover(|pointer_pos| {
        pointer_position = Some(pointer_pos);
        if canvas.input().pointer.primary_down() {
            canvas.for_each_integration_mut(|mut integration| {
                integration.focus_closest_sample(&pointer_pos.into());
            });
        }
    });
    let show_velocity = canvas.input().modifiers.alt;

    canvas.for_each_integration(|integration| {
        if let Some((ref_sample, calc_sample)) = integration.focussed_sample() {
            // Draw all sample points. Highlighted points will be re-painted below.
            for position in calc_sample.positions_iter() {
                canvas.draw_sample_point(position, &settings.point_formats.other_position);
            }

            if show_velocity {
                let velocity_to_explain = pointer_position.map_or_else(
                    || calc_sample.last_computed_velocity(),
                    |pos| calc_sample.closest_computed_velocity(pos),
                );
                if velocity_to_explain == calc_sample.last_computed_velocity() {
                    highlight_reference_velocity(canvas, ref_sample, settings);
                } else {
                    highlight_reference_velocity(
                        canvas,
                        &canvas.scenario().borrow().calc_intermediate_sample(
                            &ref_sample.get_start_condition(),
                            velocity_to_explain.sampling_position().dt_fraction()
                                * calc_sample.dt(),
                        ),
                        settings,
                    );
                }
                explain_derived_velocity(&velocity_to_explain, calc_sample.dt(), canvas, settings);
            } else {
                let position_to_explain = pointer_position.map_or_else(
                    || calc_sample.last_computed_position(),
                    |pos| calc_sample.closest_computed_position(pos),
                );
                // highlight the ref. position that corresponds to `position_to_explain`
                if position_to_explain == calc_sample.last_computed_position() {
                    highlight_reference_position(canvas, ref_sample, settings);
                } else {
                    highlight_reference_position(
                        canvas,
                        // calculate reference sample corresponding to position_to_explain:
                        &canvas.scenario().borrow().calc_intermediate_sample(
                            &ref_sample.get_start_condition(),
                            position_to_explain.dt_fraction() * calc_sample.dt(),
                        ),
                        settings,
                    );
                };
                // draw contributing vectors
                explain_derived_position(&position_to_explain, calc_sample.dt(), canvas, settings);
            }
        }
    });
}

fn explain_derived_position(
    position: &computed::position::Abstraction,
    dt: Duration,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    // draw vectors first...
    for contribution in position.contributions_iter() {
        draw_contributions_recursively(&contribution, dt, 1, canvas, settings);
        // contributions of kind `_::Position` do not return a vector
        if let Some(vector) = contribution.vector() {
            canvas.draw_vector(
                contribution.sampling_position(),
                vector,
                settings.strokes.for_contribution(contribution.kind()),
            );
        }
    }

    // ...then contributing positions on top...
    for contribution in position.contributions_iter() {
        if contribution.vector().is_none() {
            canvas.draw_sample_point(
                contribution.sampling_position(),
                &settings.point_formats.start_position,
            );
        }
    }
    // ...and finally the derived position itself:
    canvas.draw_sample_point(position.s(), &settings.point_formats.derived_position);
}

//todo: reduce number of parameters by turning this into a member function
fn draw_contributions_recursively(
    contribution: &dyn Contribution,
    factor: Duration,
    recursion_count: usize,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    let factor = factor * contribution.contributions_factor();
    for next_level_contribution in contribution.contributions_iter() {
        draw_contributions_recursively(
            &*next_level_contribution,
            factor,
            recursion_count + 1,
            canvas,
            settings,
        );
        if let Some(vector) = next_level_contribution.vector() {
            canvas.draw_vector(
                next_level_contribution.sampling_position(),
                vector * f32::from(factor),
                settings
                    .strokes
                    .for_contribution(next_level_contribution.kind())
                    .modified_for_level(recursion_count),
            );
        }
    }
}

fn explain_derived_velocity(
    velocity: &computed::velocity::Abstraction,
    dt: Duration,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    let dt = f32::from(dt) * velocity.sampling_position().dt_fraction();
    //todo: this is an ad-hoc implementation. In future, it may need a recursion as in
    //      `explain_derived_position()`.
    for contribution in velocity.contributions_iter() {
        if let Some(vector) = contribution.vector() {
            canvas.draw_vector(
                contribution.sampling_position(),
                vector * dt,
                match contribution.kind() {
                    PhysicalQuantityKind::Position => {
                        panic!("A position is not expected to contribute to a velocity")
                    }
                    PhysicalQuantityKind::Velocity => settings.strokes.start_velocity,
                    PhysicalQuantityKind::Acceleration => {
                        settings.strokes.contributing_acceleration
                    }
                },
            );
        }
    }
    canvas.draw_vector(
        velocity.sampling_position().s(),
        velocity.v() * dt,
        settings.strokes.derived_velocity,
    );
}

fn highlight_reference_position(
    canvas: &CanvasPainter,
    reference_sample: &Step,
    settings: &Settings,
) {
    canvas.draw_sample_point(
        reference_sample.last_computed_position().s(),
        &settings.point_formats.reference_position,
    );
}

fn highlight_reference_velocity(canvas: &CanvasPainter, ref_sample: &Step, settings: &Settings) {
    canvas.draw_vector(
        ref_sample.last_s(),
        ref_sample.last_v() * ref_sample.dt(),
        settings.strokes.reference_velocity,
    );
}
