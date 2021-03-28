use super::{BUTTON_GLYPH_ADD, BUTTON_GLYPH_DELETE};
use crate::prelude::*;
use egui::{
    color_picker::{color_edit_button_hsva, Alpha},
    Slider,
};

enum StepSizeOperation {
    Noop,
    Create,
    Delete(bevy_ecs::Entity),
}

pub fn show(ui: &mut Ui, world: &mut World) {
    ui.heading("Step Sizes");
    let operation = show_step_size_table(ui, world);
    match operation {
        StepSizeOperation::Create => {}
        StepSizeOperation::Delete(_) => {}
        StepSizeOperation::Noop => (),
    }
}

fn show_step_size_table(ui: &mut Ui, world: &mut World) -> StepSizeOperation {
    let mut operation = StepSizeOperation::Noop;

    egui::Grid::new("integrator grid")
        .striped(false)
        .show(ui, |mut ui| {
            // table header:
            if ui.small_button(BUTTON_GLYPH_ADD).clicked() {
                operation = StepSizeOperation::Create;
            }
            ui.label("Label");
            ui.label("Color");
            ui.label("Duration");
            ui.end_row();

            // table body:
            let step_sizes = world.query_mut::<(
                bevy_ecs::Entity,
                &mut step_size::comp::UserLabel,
                &mut step_size::comp::Duration,
                &mut step_size::comp::Color,
            )>();
            for (step_size_id, mut label, mut duration, mut color) in step_sizes {
                // button '-':
                if is_deletion_allowed(&step_size_id) {
                    if ui.small_button(BUTTON_GLYPH_DELETE).clicked() {
                        operation = StepSizeOperation::Delete(step_size_id);
                    }
                } else {
                    ui.label("");
                }
                // edit label:
                ui.add(egui::TextEdit::singleline(&mut label.0).desired_width(0.));
                if label.0.is_empty() {
                    label.0 = "<unnamed>".to_string();
                }
                // edit color:
                color_edit_button_hsva(&mut ui, &mut *color, Alpha::BlendOrAdditive);
                // edit dt:
                let mut dt = duration.0.get().into_inner();
                ui.add(Slider::f32(&mut dt, 0.01..=2.).logarithmic(true));
                duration.0.set(R32::from(dt).max(R32::from(0.01)));
            }
            ui.end_row();
        });
    operation
}

fn is_deletion_allowed(step_size_id: &bevy_ecs::Entity) -> bool {
    true
}
