use crate::prelude::*;
use bevy_ecs::Entity;

pub fn render(state: &ControlState, world: &mut World) {
    let canvas_ids = world
        .query::<(Entity, &canvas::comp::ScenarioId)>()
        .map(|(e, s)| (e, *s))
        .collect::<Vec<_>>();

    for (canvas_id, scenario_id) in canvas_ids {
        let (acceleration, start_position, start_velocity, duration) = world
            .get::<(
                &scenario::comp::Acceleration,
                &scenario::comp::StartPosition,
                &scenario::comp::StartVelocity,
                &scenario::comp::Duration,
            )>(scenario_id.0)
            .unwrap();
        let mut canvas_integrations = world
            .query::<(
                &integration::comp::State,
                &step_size::Entity,
                &canvas::Entity,
                &integrator::Entity,
            )>()
            .filter(|(_, _, integration_canvas_id, _)| integration_canvas_id.0 == canvas_id)
            .map(|(integration, step_size_id, _, integrator_id)| {
                let (integrator, stroke) = world
                    .get::<(&integrator::comp::Integrator, &integrator::comp::Stroke)>(
                        integrator_id.0,
                    )
                    .unwrap();
                let (step_duration, step_color) = world
                    .get::<(&step_size::comp::Duration, &step_size::comp::Color)>(step_size_id.0)
                    .unwrap();
                (integration, integrator, step_duration, step_color, stroke)
            })
            .collect::<Vec<_>>();
        let min_dt = canvas_integrations
            .iter()
            .map(|(_, _, step_duration, _, _)| step_duration.0.get())
            .min() // this crate depends on decorum::R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let canvas = world
            .get_mut::<&mut canvas::comp::State>(canvas_id)
            .unwrap();
        let first_time = !canvas.has_trajectory();
        canvas.update_trajectory(
            &***acceleration,
            start_position,
            start_velocity,
            duration,
            min_dt,
        );
        for (integration, integrator, step_duration, _, _) in &mut canvas_integrations {
            integration.update(
                &***acceleration,
                start_position,
                start_velocity,
                duration,
                &****integrator,
                *step_duration,
            );
        }
        if first_time {
            let mut bbox = canvas.bbox();
            canvas_integrations
                .iter()
                .for_each(|(integration, _, _, _, _)| integration.stretch_bbox(&mut bbox));
            canvas.set_visible_bbox(&bbox);
        }

        canvas.draw_trajectory(state.strokes.trajectory);
        for (ref mut integration, _, _, &step_color, &stroke) in &mut canvas_integrations {
            integration.draw_on(&mut canvas, Color32::from(*step_color), *stroke);
        }
    }
}
