use crate::prelude::*;
use ::bevy::ecs::Query;
use ::egui::{stroke_ui, Ui};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, integrators: &mut Query<(&Box<dyn Integrator>, &mut Stroke)>) {
    ui.heading("Integrators");
    for (integrator, mut stroke) in integrators.iter_mut() {
        stroke_ui(ui, &mut stroke, &(*integrator.label()));
    }
}
