use super::{
    constants,
    core::{Duration, Obj},
    entities::StepSize,
    misc::UserLabel,
    ui_import::{
        egui,
        egui::{
            color_picker::{color_edit_button_hsva, Alpha},
            Slider, TextEdit,
        },
        Color32, Hsva, Ui,
    },
    World,
};
use ::std::rc::Rc;

enum Operation {
    Noop,
    Create,
    Delete(Obj<StepSize>),
    SetDuration(Obj<StepSize>, Duration),
    SetColor(Obj<StepSize>, Color32),
    SetLabel(Obj<StepSize>, String),
}

pub fn show(ui: &mut Ui, world: &mut World) {
    let operation = show_step_size_table(ui, world);
    match operation {
        Operation::Create => {
            world.add_step_size(StepSize {
                user_label: UserLabel("".into()),
                duration: 0.5.into(),
                color: Color32::default(),
            });
        }
        Operation::Delete(step_size) => {
            world.remove_step_size(step_size);
        }
        Operation::SetDuration(step_size, new_duration) => {
            step_size.borrow_mut().duration = new_duration.max(0.01.into());
        }
        Operation::SetColor(step_size, new_color) => {
            step_size.borrow_mut().color = new_color;
        }
        Operation::SetLabel(step_size, new_label) => {
            step_size.borrow_mut().user_label.0 = new_label;
        }
        Operation::Noop => (),
    }
}

fn show_step_size_table(ui: &mut Ui, world: &World) -> Operation {
    let mut operation = Operation::Noop;

    egui::Grid::new("integrator grid")
        .striped(false)
        .show(ui, |ui| {
            // table header:
            if ui.small_button(constants::BUTTON_GLYPH_ADD).clicked() {
                operation = Operation::Create;
            }
            ui.label("Step Size (dt)");
            ui.label("Color");
            ui.label("Label");
            ui.end_row();

            // table body:
            world.step_sizes().for_each(|step_size| {
                // button '-':
                if is_deletion_allowed(step_size, world) {
                    if ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                        operation = Operation::Delete(Rc::clone(step_size));
                    }
                } else {
                    ui.label("");
                }
                // edit dt:
                let mut dt = step_size.borrow().duration.into();
                if ui
                    .add(Slider::new(&mut dt, 0.01..=2.).logarithmic(true))
                    .changed()
                {
                    operation = Operation::SetDuration(Rc::clone(step_size), dt.into());
                };
                // edit color:
                let mut color: Hsva = step_size.borrow().color.into();
                if color_edit_button_hsva(ui, &mut color, Alpha::BlendOrAdditive).changed() {
                    operation = Operation::SetColor(Rc::clone(step_size), color.into());
                }
                // edit label:
                let mut label = step_size.borrow().user_label.clone();
                if ui
                    .add(TextEdit::singleline(&mut label).desired_width(20.))
                    .changed()
                {
                    operation = Operation::SetLabel(Rc::clone(step_size), label);
                }

                ui.end_row();
            });
        });
    operation
}

fn is_deletion_allowed(step_size: &Obj<StepSize>, world: &World) -> bool {
    !world.canvases().any(|canvas| {
        canvas
            .borrow()
            .integrations()
            .any(|integration| Rc::ptr_eq(step_size, &integration.borrow().step_size))
    })
}
