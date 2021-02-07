use crate::{Canvas, ConfiguredIntegrator, Integration, Scenario, StepSize};
use bevy::prelude::*;
use egui::Color32;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(render_integration.system());
    }
}

pub fn render_integration(
    mut integrations: Query<&mut Integration>,
    mut canvases: Query<&mut Canvas>,
    step_sizes: Query<&StepSize>,
    scenarios: Query<&Scenario>,
    integrators: Query<&ConfiguredIntegrator>,
) {
    for mut integration in integrations.iter_mut() {
        let step_size = step_sizes.get(integration.get_step_size_id()).unwrap();
        let canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();
        let scenario = canvas.get_scenario(&scenarios).unwrap();
        let integrator = integrators.get(integration.get_integrator_id()).unwrap();
        let integration_steps = integrator.integrate(&scenario, step_size.dt);
        let reference_samples = scenario.calculate_reference_samples(step_size.dt);
        integration.set_reference_samples(reference_samples);
        integration.set_integration_steps(integration_steps);
        integration.draw_on(
            &canvas,
            Color32::from(step_size.color),
            Color32::from(integration.color),
        );
    }
}
