use super::{
    constants, core, entities, misc, ui_import,
    ui_import::{egui::CollapsingHeader, Ui},
    World,
};

mod integrators;
mod layers;
mod scenarios;
mod step_sizes;

pub fn show(ui: &mut Ui, world: &mut World) {
    ui.collapsing("Layer Visibility", |ui| {
        layers::show(ui, world);
    });
    CollapsingHeader::new("Scenarios")
        .default_open(true)
        .show(ui, |ui| {
            scenarios::show(ui, world);
        });
    CollapsingHeader::new("Integrators")
        .default_open(true)
        .show(ui, |ui| {
            integrators::show(ui, world);
        });
    CollapsingHeader::new("Step Sizes")
        .default_open(true)
        .show(ui, |ui| step_sizes::show(ui, world));
}
