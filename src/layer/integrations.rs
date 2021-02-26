use crate::{
    canvas,
    integrator::{self, Integrator},
    scenario, step_size, Canvas, Integration, Scenario, StepSize, UiState,
};
use bevy::prelude::*;
use egui::Stroke;

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
    integrators: Query<(&Box<dyn Integrator>, &Stroke)>,
    //integrators: Query<&integrator::Bundle>,
    step_sizes: Query<&StepSize>,
    scenarios: Query<&Scenario>,
) {
    for (canvas_id, mut canvas, scenario_id) in canvases.iter_mut() {
        let scenario = scenarios.get(scenario_id.0).unwrap();
        let mut canvas_integrations = integrations
            .iter_mut()
            .filter(|(_, _, integration_canvas_id, _)| integration_canvas_id.0 == canvas_id)
            .map(|(integration, step_size_id, _, integrator_id)| {
                let todo = "create a bundle type with named components for this tuple";
                let (integrator, stroke) = integrators.get(integrator_id.0).unwrap();
                let step_size = step_sizes.get(step_size_id.0).unwrap();
                (integration, integrator, step_size, stroke)
            })
            .collect::<Vec<_>>();
        let min_dt = canvas_integrations
            .iter()
            .map(|(_, _, step_size, _)| step_size.dt.get())
            .min() // this crate depends on decorum::R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let first_time = !canvas.has_trajectory();
        canvas.update_trajectory(&scenario, min_dt);
        for (ref mut integration, integrator, step_size, _) in &mut canvas_integrations {
            integration.update(&scenario, &***integrator, &step_size);
        }
        if first_time {
            let mut bbox = canvas.bbox();
            canvas_integrations
                .iter()
                .for_each(|(integration, _, _, _)| integration.stretch_bbox(&mut bbox));
            canvas.set_visible_bbox(&bbox);
        }

        canvas.draw_trajectory(ui_state.strokes.trajectory);
        for (ref mut integration, _, step_size, &stroke) in &mut canvas_integrations {
            integration.draw_on(&mut canvas, step_size.color.into(), stroke);
        }
    }
}
