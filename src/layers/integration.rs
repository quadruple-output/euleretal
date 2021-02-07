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
        let mut canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();
        let scenario = canvas.get_scenario(&scenarios).unwrap();
        let integrator = integration.get_integrator(&integrators).unwrap();
        let step_size = integration.get_step_size(&step_sizes).unwrap();

        integration.update(&scenario, &integrator, &step_size);
        integration.draw_on(
            &mut canvas,
            Color32::from(step_size.color),
            integrator.stroke,
        );
    }
}
