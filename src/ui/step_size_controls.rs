use crate::prelude::*;
use bevy_ecs::World;
use egui::{
    color_picker::{color_edit_button_hsva, Alpha},
    Slider, Ui,
};

pub fn show(ui: &mut Ui, world: &mut World) {
    let step_sizes = world.query_mut::<(
        &mut step_size::comp::UserLabel,
        &mut step_size::comp::Duration,
        &mut step_size::comp::Color,
    )>();
    ui.heading("Step Sizes");
    for (mut label, mut duration, mut color) in step_sizes {
        ui.horizontal(|mut ui| {
            // edit color:
            color_edit_button_hsva(&mut ui, &mut *color, Alpha::BlendOrAdditive);
            // edit label:
            ui.add(egui::TextEdit::singleline(&mut label.0).desired_width(0.));
            if label.0.is_empty() {
                label.0 = "<unnamed>".to_string();
            }
            // edit dt:
            let mut dt = duration.0.get().into_inner();
            ui.add(Slider::f32(&mut dt, 0.01..=2.).text("dt").logarithmic(true));
            duration.0.set(R32::from(dt).max(R32::from(0.01)));
        });
    }
}
