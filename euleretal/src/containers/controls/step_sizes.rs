use super::{
    constants,
    core::Duration,
    entities::StepSize,
    misc::{entity_store, UserLabel},
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

enum Operation {
    Noop,
    Create,
    Delete(entity_store::Index<StepSize>),
    SetDuration(entity_store::Index<StepSize>, Duration),
    SetColor(entity_store::Index<StepSize>, Color32),
    SetLabel(entity_store::Index<StepSize>, String),
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
        Operation::SetDuration(step_size_idx, new_duration) => {
            world[step_size_idx].borrow_mut().duration = new_duration.max(0.01.into());
        }
        Operation::SetColor(step_size_idx, new_color) => {
            world[step_size_idx].borrow_mut().color = new_color;
        }
        Operation::SetLabel(step_size_idx, new_label) => {
            world[step_size_idx].borrow_mut().user_label.0 = new_label;
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
            world
                .step_sizes()
                .enumerate()
                .for_each(|(each_step_size_idx, each_step_size)| {
                    // button '-':
                    if is_deletion_allowed(each_step_size_idx, world) {
                        if ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                            operation = Operation::Delete(each_step_size_idx);
                        }
                    } else {
                        ui.label("");
                    }
                    // edit dt:
                    let mut dt = each_step_size.borrow().duration.into();
                    if ui
                        .add(Slider::new(&mut dt, 0.01..=2.).logarithmic(true))
                        .changed()
                    {
                        operation = Operation::SetDuration(each_step_size_idx, dt.into());
                    };
                    // edit color:
                    let mut color: Hsva = each_step_size.borrow().color.into();
                    if color_edit_button_hsva(ui, &mut color, Alpha::BlendOrAdditive).changed() {
                        operation = Operation::SetColor(each_step_size_idx, color.into());
                    }
                    // edit label:
                    let mut label = each_step_size.borrow().user_label.clone();
                    if ui
                        .add(TextEdit::singleline(&mut label).desired_width(20.))
                        .changed()
                    {
                        operation = Operation::SetLabel(each_step_size_idx, label);
                    }

                    ui.end_row();
                });
        });
    operation
}

fn is_deletion_allowed(step_size_idx: entity_store::Index<StepSize>, world: &World) -> bool {
    !world.canvases().any(|canvas| {
        canvas
            .borrow()
            .integrations()
            .any(|integration| step_size_idx == integration.borrow().step_size_idx())
    })
}
