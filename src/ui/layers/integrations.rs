use crate::prelude::*;

pub fn render(
    world: &mut World,
    state: &ControlState,
    canvas_id: bevy_ecs::Entity,
    painter: &egui::Painter,
) {
    let scenario_id = world.get::<canvas::comp::ScenarioId>(canvas_id).unwrap();
    let scenario = scenario_id.gather_from(world);

    let mut integrations: Vec<integration::Gathered> = world
        .query::<integration::Query>()
        .map(|integration| integration.gather_from(world))
        .filter(|integration| integration.canvas_id == canvas_id)
        .collect::<Vec<_>>();

    let min_dt = integrations
        .iter()
        .map(|integration| integration.step_duration.get())
        .min() // this crate depends on decorum::R32 just to be able to use this min() function
        .unwrap_or_else(|| 0.1.into());

    //let mut canvas = world.get_mut::<canvas::comp::State>(canvas_id).unwrap();
    let mut canvas = unsafe {
        // should be safe as long as there is no other access to the
        // canvas::comp::State of this entity
        world
            .get_mut_unchecked::<canvas::comp::State>(canvas_id)
            .unwrap()
    };

    let first_time = !canvas.has_trajectory();
    canvas.update_trajectory(
        scenario.acceleration,
        &scenario.start_position,
        &scenario.start_velocity,
        &scenario.duration,
        min_dt,
    );
    for integration in &mut integrations {
        if first_time {
            integration.reset();
        }
        integration.update(
            scenario.acceleration,
            &scenario.start_position,
            &scenario.start_velocity,
            &scenario.duration,
            integration.integrator,
            &integration.step_duration,
        );
    }
    if first_time {
        let mut bbox = canvas.bbox();
        integrations
            .iter()
            .for_each(|integration| integration.stretch_bbox(&mut bbox));
        canvas.set_visible_bbox(&bbox);
    }

    canvas.draw_trajectory(state.strokes.trajectory, painter);
    for integration in &mut integrations {
        integration.draw_on(
            &canvas,
            Color32::from(integration.step_color),
            *integration.stroke,
            painter,
        );
    }
}
