use crate::prelude::*;
use bevy_ecs::Entity;

pub fn render(world: &mut World, state: &ControlState) {
    let canvas_and_scenario_ids = world
        .query::<(Entity, &canvas::comp::ScenarioId)>()
        .map(|(canvas_id, scenario_id)| (canvas_id, scenario_id.0))
        .collect::<Vec<_>>();

    let integrations = world
        .query::<integration::Query>()
        .map(|integration| integration.gather_from(&world))
        .collect::<Vec<_>>();

    let todo = "introduce scenarios::Query (like above)";
    let scenarios = world
        .query::<(
            Entity,
            &scenario::comp::Acceleration,
            &scenario::comp::StartPosition,
            &scenario::comp::StartVelocity,
            &scenario::comp::Duration,
        )>()
        .map(|(id, a, pos, v, d)| {
            (
                id,
                &*a,
                pos.0.copy_read_only(),
                v.0.copy_read_only(),
                d.0.copy_read_only(),
            )
        })
        .collect::<Vec<_>>();

    for (canvas_id, scenario_id) in canvas_and_scenario_ids {
        let (_, acceleration, start_position, start_velocity, duration) = scenarios
            .iter()
            .find(|(id, ..)| *id == scenario_id)
            .unwrap();
        let mut canvas_integrations = integrations
            .iter()
            .filter(|integration| integration.canvas_id == canvas_id)
            .collect::<Vec<_>>();
        let min_dt = canvas_integrations
            .iter()
            .map(|integration| integration.step_duration.get())
            .min() // this crate depends on decorum::R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let mut canvas = unsafe {
            // should be safe as long as there is no other access to the
            // canvas::comp::State of this entity
            world
                .get_mut_unchecked::<canvas::comp::State>(canvas_id)
                .unwrap()
        };
        let first_time = !canvas.has_trajectory();
        canvas.update_trajectory(
            &***acceleration,
            start_position,
            start_velocity,
            duration,
            min_dt,
        );
        for integration in &mut canvas_integrations {
            integration.update(
                &***acceleration,
                start_position,
                start_velocity,
                duration,
                integration.integrator,
                &integration.step_duration,
            );
        }
        if first_time {
            let mut bbox = canvas.bbox();
            canvas_integrations
                .iter()
                .for_each(|integration| integration.stretch_bbox(&mut bbox));
            canvas.set_visible_bbox(&bbox);
        }

        canvas.draw_trajectory(state.strokes.trajectory);
        for integration in &mut canvas_integrations {
            integration.draw_on(
                &mut canvas,
                Color32::from(integration.step_color),
                *integration.stroke,
            );
        }
    }
}
