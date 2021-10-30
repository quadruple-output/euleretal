use super::{Ui, World};

pub fn show(ui: &mut Ui, world: &mut World) {
    ui.vertical(|ui| {
        ui.checkbox(&mut world.settings.layerflags.coordinates, "Coordinates");
        ui.checkbox(
            &mut world.settings.layerflags.acceleration_field,
            "Acceleration Field",
        );
        ui.checkbox(&mut world.settings.layerflags.inspector, "Inspector");
    });
}
