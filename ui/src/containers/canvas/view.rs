use super::{
    constants,
    core::Obj,
    entities::{Canvas, Integration, Integrator, ObjExtras, StepSize},
    layers,
    misc::entity_store,
    ui_import::{
        egui::{self, Layout},
        Pos2, Ui, Vec2,
    },
    World,
};
use ::std::{cell::RefCell, rc::Rc};

enum IntegrationOperation {
    Noop,
    Create,
    Delete {
        integration: Obj<Integration>,
    },
    SetIntegrator {
        integration: Obj<Integration>,
        integrator: entity_store::Index<Integrator>,
    },
    SetStepSize {
        integration: Obj<Integration>,
        step_size: Obj<StepSize>,
    },
}

pub enum CanvasOperation<'a> {
    Noop,
    Create { source_canvas: &'a RefCell<Canvas> },
    Close { canvas: *const RefCell<Canvas> },
}

pub fn show_canvas(ui: &mut Ui, canvas: &RefCell<Canvas>, size: Vec2, world: &World) {
    let mut canvas_painter = canvas.allocate_painter(ui, size);

    canvas_painter.pan_and_zoom();
    if world.settings.layerflags.coordinates {
        layers::coordinates::render(&world.settings.strokes, &canvas_painter);
    }
    if world.settings.layerflags.acceleration_field {
        layers::acceleration_field::render(&canvas_painter, world);
    }
    layers::integrations::render(&mut canvas_painter, world);
    if world.settings.layerflags.inspector {
        layers::inspector::render(&canvas_painter, world);
    }
}

/// returns the `CanvasOperation` as `inner`
pub fn show_header_bar<'a>(
    ui: &mut Ui,
    canvas: &'a RefCell<Canvas>,
    world: &World,
    can_close: bool,
    can_create: bool,
) -> egui::InnerResponse<CanvasOperation<'a>> {
    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            show_scenario_selector(ui, canvas, world);
            show_integration_selector(ui, canvas, world);
        });
        ui.with_layout(Layout::right_to_left(), |ui| {
            let mut operation = CanvasOperation::Noop;
            if can_close && ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                operation = CanvasOperation::Close { canvas };
            }
            if can_create && ui.small_button(constants::BUTTON_GLYPH_ADD).clicked() {
                operation = CanvasOperation::Create {
                    source_canvas: canvas,
                };
            }
            operation
        })
        .inner
    })
}

fn show_scenario_selector(ui: &mut Ui, canvas: &RefCell<Canvas>, world: &World) {
    let selector_id = ui.make_persistent_id(format!("scenario_selector_{:?}", canvas.as_ptr()));
    let canvas_scenario_idx = canvas.borrow().scenario_idx();
    let canvas_scenario = &world.scenarios()[canvas_scenario_idx];
    let mut selected_scenario_idx = canvas_scenario_idx;
    egui::ComboBox::from_id_source(selector_id)
        .selected_text(canvas_scenario.borrow().label())
        .show_ui(ui, |ui| {
            world
                .scenarios()
                .enumerate()
                .for_each(|(each_idx, each_scenario)| {
                    ui.selectable_value(
                        &mut selected_scenario_idx,
                        each_idx,
                        each_scenario.borrow().label(),
                    );
                });
        });
    if selected_scenario_idx != canvas_scenario_idx {
        canvas.borrow_mut().set_scenario(selected_scenario_idx);
    }
}

fn show_integration_selector(ui: &mut Ui, canvas: &RefCell<Canvas>, world: &World) {
    let mut window_is_open = canvas.borrow().ui_integrations_window_is_open;
    let button_response = ui.add(egui::Button::new("Integrations"));
    if button_response.clicked() {
        window_is_open = !window_is_open;
    };
    let operation = show_integrations_pop_up(
        ui,
        ui.make_persistent_id(format!("integrations_button_{:?}", canvas.as_ptr())),
        &mut window_is_open,
        Pos2::new(button_response.rect.left(), button_response.rect.bottom()),
        canvas,
        world,
    );
    canvas.borrow_mut().ui_integrations_window_is_open = window_is_open;

    match operation {
        IntegrationOperation::Create => {
            canvas.borrow_mut().add_integration(Integration::new(
                entity_store::Index::default(), // Index to the first entry in the list
                Rc::clone(world.step_sizes().next().unwrap()),
            ));
        }
        IntegrationOperation::Delete { integration } => {
            canvas.borrow_mut().remove_integration(integration);
        }
        IntegrationOperation::SetIntegrator {
            integration,
            integrator,
        } => {
            integration.borrow_mut().set_integrator(integrator);
        }
        IntegrationOperation::SetStepSize {
            integration,
            step_size,
        } => {
            integration.borrow_mut().set_step_size(step_size);
        }
        IntegrationOperation::Noop => (),
    }
}

fn show_integrations_pop_up(
    ui: &mut Ui,
    id: egui::Id,
    open: &mut bool,
    default_pos: Pos2,
    canvas: &RefCell<Canvas>,
    world: &World,
) -> IntegrationOperation {
    let mut operation = IntegrationOperation::Noop;

    egui::Window::new("Integrations")
        .id(id)
        .open(open)
        .collapsible(false)
        .default_pos(default_pos)
        .show(ui.ctx(), |ui| {
            egui::Grid::new("integrator grid")
                .striped(false)
                .show(ui, |ui| {
                    // table header:
                    if ui.small_button(constants::BUTTON_GLYPH_ADD).clicked() {
                        operation = IntegrationOperation::Create;
                    }
                    ui.label("Line");
                    ui.label("Integrator");
                    ui.label("Step Size");
                    ui.end_row();

                    // table body:
                    let num_integrations = canvas.borrow().integrations().len();
                    canvas.borrow().integrations().for_each(|integration| {
                        if num_integrations > 1 {
                            if ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                                operation = IntegrationOperation::Delete {
                                    integration: Rc::clone(integration),
                                };
                            }
                        } else {
                            ui.label("");
                        }
                        let integrator_stroke =
                            world[integration.borrow().integrator_idx()].borrow().stroke;
                        super::misc::my_stroke_preview(
                            ui,
                            integrator_stroke,
                            Some((
                                &world.settings.point_formats.derived_position,
                                integration.borrow().get_step_color(),
                            )),
                        );
                        // wrappind the combobox in a horizontal ui help aligning the grid
                        ui.horizontal(|ui| {
                            if let Some(integrator) =
                                show_integrator_selector(ui, integration, world)
                            {
                                operation = IntegrationOperation::SetIntegrator {
                                    integration: Rc::clone(integration),
                                    integrator,
                                };
                            }
                        });
                        if let Some(step_size) = show_step_size_selector(ui, integration, world) {
                            operation = IntegrationOperation::SetStepSize {
                                integration: Rc::clone(integration),
                                step_size,
                            };
                        }
                        ui.end_row();
                    });
                });
        });
    operation
}

fn show_integrator_selector(
    ui: &mut Ui,
    integration: &Obj<Integration>,
    world: &World,
) -> Option<entity_store::Index<Integrator>> {
    let current_integrator_idx = integration.borrow().integrator_idx();
    let current_integrator = &world[current_integrator_idx];
    let mut selected_integrator_idx = current_integrator_idx;

    egui::ComboBox::from_id_source(
        ui.make_persistent_id(format!("integrator_selector_{:?}", integration.as_ptr())),
    )
    .selected_text(current_integrator.borrow().core.label())
    .show_ui(ui, |ui| {
        world
            .integrators()
            .enumerate()
            .for_each(|(each_idx, each_integrator)| {
                let each_core_integrator = &each_integrator.borrow().core;
                ui.selectable_value(
                    &mut selected_integrator_idx,
                    each_idx,
                    each_core_integrator.label(),
                )
                .on_hover_text(each_core_integrator.description());
            });
    })
    .response
    .on_hover_text(current_integrator.borrow().core.description());

    if selected_integrator_idx == current_integrator_idx {
        None
    } else {
        Some(selected_integrator_idx)
    }
}

fn show_step_size_selector(
    ui: &mut Ui,
    integration: &Obj<Integration>,
    world: &World,
) -> Option<Obj<StepSize>> {
    let mut selected_step_size_ptr = integration.borrow().step_size.as_ptr();
    egui::ComboBox::from_id_source(
        ui.make_persistent_id(format!("step_size_selector_{:?}", integration.as_ptr())),
    )
    .selected_text(format!("{}", integration.borrow().step_size.borrow()))
    .show_ui(ui, |ui| {
        world.step_sizes().for_each(|selectable_step_size| {
            ui.selectable_value(
                &mut selected_step_size_ptr,
                selectable_step_size.as_ptr(),
                format!("{}", selectable_step_size.borrow()),
            );
        });
    });

    if selected_step_size_ptr == integration.borrow().step_size.as_ptr() {
        None
    } else {
        world
            .step_sizes()
            .find(|candidate| selected_step_size_ptr == candidate.as_ptr())
            .map(Rc::clone)
    }
}
