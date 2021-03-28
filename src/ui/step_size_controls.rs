use super::{BUTTON_GLYPH_ADD, BUTTON_GLYPH_DELETE};
use crate::prelude::*;
use egui::{
    color_picker::{color_edit_button_hsva, Alpha},
    Slider, TextEdit,
};

enum Operation {
    Noop,
    Create,
    Delete(bevy_ecs::Entity),
    SetDuration(bevy_ecs::Entity, R32),
    SetColor(bevy_ecs::Entity, Hsva),
    SetLabel(bevy_ecs::Entity, String),
}

pub fn show(ui: &mut Ui, world: &mut World) {
    ui.heading("Step Sizes");
    let operation = show_step_size_table(ui, world);
    match operation {
        Operation::Create => {
            step_size::Bundle(
                step_size::Kind,
                UserLabel("<unnamed>".into()),
                Duration(ChangeTracker::with(0.5.into())),
                step_size::comp::Color::default(),
            )
            .spawn(world);
        }
        Operation::Delete(step_size_id) => {
            world.despawn(step_size_id).unwrap();
        }
        Operation::SetDuration(step_size_id, new_duration) => {
            let mut step_duration = world
                .get_mut::<step_size::comp::Duration>(step_size_id)
                .unwrap();
            step_duration.0.set(new_duration.max(0.01.into()));
        }
        Operation::SetColor(step_size_id, new_color) => {
            let mut step_color = world
                .get_mut::<step_size::comp::Color>(step_size_id)
                .unwrap();
            *step_color = new_color;
        }
        Operation::SetLabel(step_size_id, new_label) => {
            let mut step_label = world
                .get_mut::<step_size::comp::UserLabel>(step_size_id)
                .unwrap();
            step_label.0 = new_label;
        }
        Operation::Noop => (),
    }
}

fn show_step_size_table(ui: &mut Ui, world: &World) -> Operation {
    let mut operation = Operation::Noop;

    egui::Grid::new("integrator grid")
        .striped(false)
        .show(ui, |mut ui| {
            // table header:
            if ui.small_button(BUTTON_GLYPH_ADD).clicked() {
                operation = Operation::Create;
            }
            ui.label("Duration");
            ui.label("Color");
            ui.label("Label");
            ui.end_row();

            // table body:
            for step_size in world
                .query::<step_size::Query>()
                .map(|step_size| step_size.gather_from(world))
            {
                // button '-':
                if is_deletion_allowed(step_size.id, world) {
                    if ui.small_button(BUTTON_GLYPH_DELETE).clicked() {
                        operation = Operation::Delete(step_size.id);
                    }
                } else {
                    ui.label("");
                }
                // edit dt:
                let mut dt = step_size.duration.get().into_inner();
                if ui
                    .add(Slider::f32(&mut dt, 0.01..=2.).logarithmic(true))
                    .changed()
                {
                    operation = Operation::SetDuration(step_size.id, dt.into());
                };
                // edit color:
                let mut color = step_size.color;
                if color_edit_button_hsva(&mut ui, &mut color, Alpha::BlendOrAdditive).changed() {
                    operation = Operation::SetColor(step_size.id, color);
                }
                // edit label:
                let mut label = step_size.label.clone();
                if ui
                    .add(TextEdit::singleline(&mut label).desired_width(20.))
                    .changed()
                {
                    operation = Operation::SetLabel(step_size.id, label);
                }

                ui.end_row();
            }
        });
    operation
}

fn is_deletion_allowed(step_size_id: bevy_ecs::Entity, world: &World) -> bool {
    world
        .query::<(&integration::Kind, &integration::comp::StepSizeId)>()
        .find(|(_, candidate_step_size_id)| candidate_step_size_id.0 == step_size_id)
        .is_none()
}
