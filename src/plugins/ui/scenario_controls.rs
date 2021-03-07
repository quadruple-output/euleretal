use crate::prelude::*;
use ::bevy::ecs::Query;
use ::egui::{Slider, Ui};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, scenarios: &mut Query<(&Box<dyn Acceleration>, &mut Duration)>) {
    ui.heading("Scenarios");
    for (acceleration, mut duration) in scenarios.iter_mut() {
        ui.horizontal(|ui| {
            ui.label(acceleration.label());
            ui.vertical(|ui| {
                let mut duration_for_edit = duration.0.get().into_inner();
                ui.add(
                    Slider::f32(&mut duration_for_edit, 0.1..=50.)
                        .logarithmic(true)
                        .text("duration"),
                );
                duration.0.set(duration_for_edit.into());
            });
        });
    }
}
