use crate::prelude::*;
use bevy_ecs::World;
use egui::{Slider, Ui};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    egui::Grid::new("integrator grid")
        .striped(false)
        .show(ui, |ui| {
            // table header:
            ui.label("Duration");
            ui.label("Scenario");
            ui.end_row();

            for (acceleration, mut duration) in
                world.query_mut::<(&Box<dyn AccelerationField>, &mut Duration)>()
            {
                let mut duration_for_edit = duration.0.get().into_inner();
                ui.add(Slider::f32(&mut duration_for_edit, 0.1..=50.).logarithmic(true));
                duration.0.set(duration_for_edit.into());

                ui.label(acceleration.label());
                ui.end_row();
            }
        });
}
