use crate::{Canvas, Integration, StepSize};
use bevy::prelude::*;
use egui::Color32;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(render_integration.system());
    }
}

pub fn render_integration(
    integrations: Query<&Integration>,
    mut canvases: Query<&mut Canvas>,
    step_sizes: Query<&StepSize>,
) {
    for integration in integrations.iter() {
        let canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();
        let step_size = step_sizes.get(integration.get_step_size_id()).unwrap();
        integration.draw_on(&canvas, Color32::from(step_size.color));
    }
}
