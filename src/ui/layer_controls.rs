use super::ControlState;
use crate::egui::Ui;

pub fn show(ui: &mut Ui, state: &mut ControlState) {
    ui.heading("Layer Visibility");
    ui.vertical(|ui| {
        ui.checkbox(&mut state.layerflags.coordinates, "Coordinates");
        ui.checkbox(
            &mut state.layerflags.acceleration_field,
            "Acceleration Field",
        );
        ui.checkbox(&mut state.layerflags.inspector, "Inspector");
    });
}