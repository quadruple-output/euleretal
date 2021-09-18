use super::{
    core::{
        ComputedPosition, ComputedVelocity, Duration, Step, PhysicalQuantityKind,
        Position,
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
    position: &ComputedPosition,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    // draw vectors first...
    for contribution in position.contributions_iter() {
        match contribution.kind() {
            PhysicalQuantityKind::Position => {}
            PhysicalQuantityKind::Velocity => {
                canvas.draw_vector(
                    contribution.sampling_position(),
                    contribution.vector().unwrap(),
                    settings.strokes.contributing_velocity,
                );
            }
            PhysicalQuantityKind::Acceleration => {
                canvas.draw_vector(
                    contribution.sampling_position(),
                    contribution.vector().unwrap(),
                    settings.strokes.contributing_acceleration,
                );
            }
        }
    }
    // ...then contributing positions...
    for contribution in position.contributions_iter() {
        match contribution.kind() {
            PhysicalQuantityKind::Position => {
                canvas.draw_sample_point(
                    contribution.sampling_position(),
                    &settings.point_formats.start_position,
                );
            }
            PhysicalQuantityKind::Velocity | PhysicalQuantityKind::Acceleration => {}
        }
    }
    // ...and finally the derived position itself:
    canvas.draw_sample_point(position.s(), &settings.point_formats.derived_position);
}

fn explain_derived_velocity(
    velocity: &ComputedVelocity,
    scale: Duration,
    canvas: &CanvasPainter,
    settings: &Settings,
) {
    for contribution in velocity.contributions_iter() {
        canvas.draw_vector(
            contribution.sampling_position(),
            contribution.vector() * scale,
            match contribution.kind() {
                PhysicalQuantityKind::Position => {
                    panic!("A position is not expected to contribute to a velocity")
                }
                PhysicalQuantityKind::Velocity => settings.strokes.start_velocity,
                PhysicalQuantityKind::Acceleration => settings.strokes.contributing_acceleration,
            },
        );
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

fn highlight_reference_velocity(
    canvas: &CanvasPainter,
    ref_sample: &Step,
    settings: &Settings,
) {
    canvas.draw_vector(
        ref_sample.last_s(),
        ref_sample.last_v() * ref_sample.dt(),
        settings.strokes.reference_velocity,
    );
}
