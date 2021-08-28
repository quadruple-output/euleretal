use super::{
    core::{ComputedPosition, ComputedVelocity, Duration, IntegrationStep, PhysicalQuantityKind},
    entities::CanvasPainter,
    misc::Settings,
};

pub fn render(settings: &Settings, canvas: &CanvasPainter) {
    let mut pointer_pos_or_none = None;
    canvas.on_hover(|pointer_pos| {
        pointer_pos_or_none = Some(pointer_pos);
        if canvas.input().pointer.primary_down() {
            canvas.for_each_integration_mut(|mut integration| {
                integration.focus_closest_sample(&pointer_pos);
            });
        }
    });
    let show_velocity = canvas.input().modifiers.alt;
    canvas.for_each_integration(|integration| {
        if let Some((ref_sample, calc_sample)) = integration.focussed_sample() {
            if show_velocity {
                highlight_reference_velocity(canvas, ref_sample, settings);
                let derived_velocity = pointer_pos_or_none.map_or_else(
                    || calc_sample.last_computed_velocity(),
                    |pos| calc_sample.closest_computed_velocity(pos),
                );
                explain_derived_velocity(&derived_velocity, calc_sample.dt, canvas, settings);
            } else {
                highlight_reference_position(canvas, ref_sample, settings);
                let derived_position = pointer_pos_or_none.map_or_else(
                    || calc_sample.last_computed_position(),
                    |pos| calc_sample.closest_computed_position(pos),
                );
                explain_derived_position(&derived_position, canvas, settings);
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
    for contribution in position.contributions_iter() {
        match contribution.kind() {
            PhysicalQuantityKind::Position => {
                canvas.draw_sample_dot(
                    contribution.sampling_position(),
                    settings.colors.start_position,
                );
            }
            PhysicalQuantityKind::Velocity | PhysicalQuantityKind::Acceleration => {}
        }
    }
    canvas.draw_sample_dot(position.s(), settings.colors.derived_position);
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

fn highlight_reference_position(
    canvas: &CanvasPainter,
    ref_sample: &IntegrationStep,
    settings: &Settings,
) {
    canvas.draw_sample_dot(ref_sample.last_s(), settings.colors.derived_position);
}

fn highlight_reference_velocity(
    canvas: &CanvasPainter,
    ref_sample: &IntegrationStep,
    settings: &Settings,
) {
    canvas.draw_vector(
        ref_sample.last_s(),
        ref_sample.last_v() * ref_sample.dt,
        settings.strokes.derived_velocity,
    );
}
