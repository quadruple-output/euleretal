use super::{
    constants,
    core::Obj,
    entities::{Canvas, Integration, Integrator, ObjExtras, StepSize},
    layers,
    misc::Settings,
    ui_import::{
        egui::{self, Layout},
        Pos2, Ui, Vec2,
    },
    World,
};
use ::std::rc::Rc;

enum IntegrationOperation {
    Noop,
    Create,
    Delete {
        integration: Obj<Integration>,
    },
    SetIntegrator {
        integration: Obj<Integration>,
        integrator: Obj<Integrator>,
    },
    SetStepSize {
        integration: Obj<Integration>,
        step_size: Obj<StepSize>,
    },
}

pub enum CanvasOperation {
    Noop,
    Create { source_canvas: Obj<Canvas> },
    Close { canvas: Obj<Canvas> },
}

pub fn show_canvas(ui: &mut Ui, canvas: &Obj<Canvas>, size: Vec2, settings: &Settings) {
    let mut canvas_painter = canvas.allocate_painter(ui, size);

    canvas_painter.pan_and_zoom();
    if settings.layerflags.coordinates {
        layers::coordinates::render(&settings.strokes, &canvas_painter);
    }
    if settings.layerflags.acceleration_field {
        layers::acceleration_field::render(settings, &canvas_painter);
    }
    layers::integrations::render(&settings.strokes, &mut canvas_painter);
    if settings.layerflags.inspector {
        layers::inspector::render(settings, &canvas_painter);
    }
}

/// returns the `CanvasOperation` as `inner`
pub fn show_header_bar(
    ui: &mut Ui,
    canvas: &Obj<Canvas>,
    world: &World,
    can_close: bool,
    can_create: bool,
) -> egui::InnerResponse<CanvasOperation> {
    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            show_scenario_selector(ui, canvas, world);
            show_integration_selector(ui, canvas, world);
        });
        ui.with_layout(Layout::right_to_left(), |ui| {
            let mut operation = CanvasOperation::Noop;
            if can_close && ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                operation = CanvasOperation::Close {
                    canvas: Rc::clone(canvas),
                };
            }
            if can_create && ui.small_button(constants::BUTTON_GLYPH_ADD).clicked() {
                operation = CanvasOperation::Create {
                    source_canvas: Rc::clone(canvas),
                };
            }
            operation
        })
        .inner
    })
}

fn show_scenario_selector(ui: &mut Ui, canvas: &Obj<Canvas>, world: &World) {
    let selector_id = ui.make_persistent_id(format!("scenario_selector_{:?}", canvas.as_ptr()));
    let canvas_scenario_obj = Rc::clone(canvas.borrow().scenario());
    let mut selected_scenario_ptr = canvas_scenario_obj.as_ptr();
    let canvas_scenario = canvas_scenario_obj.borrow();
    egui::ComboBox::from_id_source(selector_id)
        .selected_text(canvas_scenario.label())
        .show_ui(ui, |ui| {
            for selectable_scenario in world.scenarios() {
                ui.selectable_value(
                    &mut selected_scenario_ptr,
                    selectable_scenario.as_ptr(),
                    selectable_scenario.borrow().label(),
                );
            }
        });
    if selected_scenario_ptr != canvas_scenario_obj.as_ptr() {
        let selected_scenario = world
            .scenarios()
            .find(|s| s.as_ptr() == selected_scenario_ptr)
            .unwrap();
        canvas
            .borrow_mut()
            .set_scenario(Rc::clone(selected_scenario));
    }
}

fn show_integration_selector(ui: &mut Ui, canvas: &Obj<Canvas>, world: &World) {
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
                Rc::clone(world.integrators().next().unwrap()),
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
    canvas: &Obj<Canvas>,
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
                        super::misc::my_stroke_preview(
                            ui,
                            integration.borrow().get_stroke(),
                            Some(integration.borrow().get_step_color().into()),
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
) -> Option<Obj<Integrator>> {
    let integration_ptr = integration.as_ptr();
    let current_integrator = &integration.borrow().integrator;
    let mut selected_integrator_ptr = current_integrator.as_ptr();

    egui::ComboBox::from_id_source(
        ui.make_persistent_id(format!("integrator_selector_{:?}", integration_ptr)),
    )
    .selected_text(current_integrator.borrow().integrator.label())
    .show_ui(ui, |ui| {
        world.integrators().for_each(|selectable_integrator| {
            ui.selectable_value(
                &mut selected_integrator_ptr,
                selectable_integrator.as_ptr(),
                selectable_integrator.borrow().integrator.label(),
            )
            .on_hover_text(selectable_integrator.borrow().integrator.description());
        });
    })
    .on_hover_text(
        integration
            .borrow()
            .integrator
            .borrow()
            .integrator
            .description(),
    );

    if selected_integrator_ptr == current_integrator.as_ptr() {
        None
    } else {
        world
            .integrators()
            .find(|candidate| candidate.as_ptr() == selected_integrator_ptr)
            .map(|found| Rc::clone(found))
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
            .map(|found| Rc::clone(found))
    }
}
