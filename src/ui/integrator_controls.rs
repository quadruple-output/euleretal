use crate::prelude::*;
use bevy_ecs::World;
use egui::{stroke_ui, Ui};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    ui.heading("Integrators");
    for (integrator, mut stroke) in world.query_mut::<(&Box<dyn Integrator>, &mut Stroke)>() {
        stroke_ui(ui, &mut stroke, &(*integrator.label()));
    }
}
