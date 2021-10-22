use super::{
    core::{
        integration_step::computed, Contribution, Duration, PhysicalQuantityKind, Position, Step,
    },
    entities::CanvasPainter,
    misc::Settings,
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
                highlight_reference_velocity(canvas, ref_sample, settings);
                explain_derived_velocity(&velocity_to_explain, calc_sample.dt(), canvas, settings);
            } else {
                let position_to_explain = pointer_position.map_or_else(
                    || calc_sample.last_computed_position(),
                    |pos| calc_sample.closest_computed_position(pos),
                );
                // highlight the ref. position that corresponds to `position_to_explain`
                highlight_reference_position(
                    canvas,
                    if position_to_explain == calc_sample.last_computed_position() {
                        ref_sample.last_s()
                    } else {
                        // calculate reference sample corresponding to position_to_explain:
                        let (s, _v) = canvas.scenario().borrow().calc_intermediate_sample(
                            &ref_sample.get_start_condition(),
                            position_to_explain.dt_fraction() * calc_sample.dt(),
                        );
                        s
                    },
                    settings,
                );
                explain_derived_position(&position_to_explain, canvas, settings);
            }
        }
    });
}

fn explain_derived_position(
    position: &computed::position::Abstraction,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    // draw vectors first...
    for contribution in position.contributions_iter() {
        // contributions of kind `_::Position` do not return a vector
        if let Some(vector) = contribution.vector() {
            canvas.draw_vector(
                contribution.sampling_position(),
                vector,
                settings.strokes.for_contribution(contribution.kind()),
            );
        }
    }
    // in case there was no contributing acceleration, draw second-level contributions to
    // velocities:
    /*
    if !position
        .contributions_iter()
        .any(|contrib| contrib.kind() == PhysicalQuantityKind::Acceleration)
    {
        for velocity_contrib in position
            .contributions_iter()
            .filter(|contrib| contrib.kind() == PhysicalQuantityKind::Velocity)
        {
            for contrib_second_order in velocity_contrib.contributions_iter() {
                if contrib_second_order.kind() == PhysicalQuantityKind::Acceleration {
                    canvas.draw_vector(
                        contrib_second_order.sampling_position(),
                        contrib_second_order.vector().unwrap(),
                        settings
                            .strokes
                            .for_contribution(contrib_second_order.kind()),
                    );
                }
            }
        }
    }
     */

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

fn explain_derived_velocity(
    velocity: &computed::velocity::Abstraction,
    scale: Duration,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    let scale = f32::from(scale);
    for contribution in velocity.contributions_iter() {
        if let Some(vector) = contribution.vector() {
            canvas.draw_vector(
                contribution.sampling_position(),
                vector * scale,
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
        velocity.sampling_position(),
        velocity.v() * scale,
        settings.strokes.derived_velocity,
    );
}

fn highlight_reference_position(canvas: &CanvasPainter, position: Position, settings: &Settings) {
    canvas.draw_sample_point(position, &settings.point_formats.reference_position);
}

fn highlight_reference_velocity(canvas: &CanvasPainter, ref_sample: &Step, settings: &Settings) {
    canvas.draw_vector(
        ref_sample.last_s(),
        ref_sample.last_v() * ref_sample.dt(),
        settings.strokes.reference_velocity,
    );
}
