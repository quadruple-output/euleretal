use crate::{canvas, integrator, scenario, step_size, Canvas, Integration, UiState};
use bevy::prelude::*;
use egui::color::Color32;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
#[allow(clippy::needless_pass_by_value, clippy::borrowed_box)]
pub fn render(
    ui_state: ResMut<UiState>,
    mut canvases: Query<(Entity, &mut Canvas, &scenario::Entity)>, // always request canvases with 'mut'
    mut integrations: Query<(
        &mut Integration,
        &step_size::Entity,
        &canvas::Entity,
        &integrator::Entity,
    )>,
    integrators: Query<(
        &integrator::Kind,
        &integrator::comp::Integrator,
        &integrator::comp::Stroke,
    )>,
    step_sizes: Query<(
        &step_size::Kind,
        &step_size::comp::Duration,
        &step_size::comp::Color,
    )>,
    scenarios: Query<(
        &scenario::Kind,
        &scenario::comp::Acceleration,
        &scenario::comp::StartPosition,
        &scenario::comp::StartVelocity,
        &scenario::comp::Duration,
    )>,
) {
    for (canvas_id, mut canvas, scenario_id) in canvases.iter_mut() {
        let (_, acceleration, start_position, start_velocity, duration) =
            scenarios.get(scenario_id.0).unwrap();
        let mut canvas_integrations = integrations
            .iter_mut()
            .filter(|(_, _, integration_canvas_id, _)| integration_canvas_id.0 == canvas_id)
            .map(|(integration, step_size_id, _, integrator_id)| {
                let (_, integrator, stroke) = integrators.get(integrator_id.0).unwrap();
                let (_, step_duration, step_color) = step_sizes.get(step_size_id.0).unwrap();
                (integration, integrator, step_duration, step_color, stroke)
            })
            .collect::<Vec<_>>();
        let min_dt = canvas_integrations
            .iter()
            .map(|(_, _, step_duration, _, _)| step_duration.0.get())
            .min() // this crate depends on decorum::R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let first_time = !canvas.has_trajectory();
        canvas.update_trajectory(
            &**acceleration,
            start_position,
            start_velocity,
            duration,
            min_dt,
        );
        for (ref mut integration, integrator, step_duration, _, _) in &mut canvas_integrations {
            integration.update(
                &**acceleration,
                start_position,
                start_velocity,
                duration,
                &***integrator,
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

        canvas.draw_trajectory(ui_state.strokes.trajectory);
        for (ref mut integration, _, _, &step_color, &stroke) in &mut canvas_integrations {
            integration.draw_on(&mut canvas, Color32::from(step_color), stroke);
        }
    }
}
