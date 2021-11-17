use super::{
    constants,
    entities::{Canvas, Integration, Integrator, ObjExtras, StepSize},
    layers,
    misc::{entity_store, my_stroke_preview},
    ui_import::{
        egui::{self, Layout},
        Pos2, Ui, Vec2,
    },
    World,
};
use ::std::cell::RefCell;

enum IntegrationOperation {
    Noop,
    Create,
    Delete {
        integration_idx: usize,
    },
    SetIntegrator {
        integration_idx: usize,
        integrator_idx: entity_store::Index<Integrator>,
    },
    SetStepSize {
        integration_idx: usize,
        step_size_idx: entity_store::Index<StepSize>,
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
                world.integrators().enumerate().next().unwrap().0,
                world.step_sizes().enumerate().next().unwrap().0,
            ));
        }
        IntegrationOperation::Delete { integration_idx } => {
            canvas.borrow_mut().remove_integration(integration_idx);
        }
        IntegrationOperation::SetIntegrator {
            integration_idx,
            integrator_idx,
        } => {
            canvas
                .borrow()
                .integration_at(integration_idx)
                .borrow_mut()
                .set_integrator(integrator_idx);
        }
        IntegrationOperation::SetStepSize {
            integration_idx,
            step_size_idx,
        } => {
            canvas
                .borrow()
                .integration_at(integration_idx)
                .borrow_mut()
                .set_step_size(step_size_idx);
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
                    for (integration_idx, integration) in canvas.borrow().integrations().enumerate()
                    {
                        if num_integrations > 1 {
                            if ui.small_button(constants::BUTTON_GLYPH_DELETE).clicked() {
                                operation = IntegrationOperation::Delete { integration_idx };
                            }
                        } else {
                            ui.label("");
                        }
                        my_stroke_preview(
                            ui,
                            world[integration.borrow().integrator_idx()].borrow().stroke,
                            Some((
                                &world.settings.point_formats.derived_position,
                                world[integration.borrow().step_size_idx()].borrow().color,
                            )),
                        );
                        // wrappind the combobox in a horizontal ui help aligning the grid
                        ui.horizontal(|ui| {
                            if let Some(integrator_idx) =
                                show_integrator_selector(ui, integration, world)
                            {
                                operation = IntegrationOperation::SetIntegrator {
                                    integration_idx,
                                    integrator_idx,
                                };
                            }
                        });
                        if let Some(step_size_idx) = show_step_size_selector(ui, integration, world)
                        {
                            operation = IntegrationOperation::SetStepSize {
                                integration_idx,
                                step_size_idx,
                            };
                        }
                        ui.end_row();
                    }
                });
        });
    operation
}

fn show_integrator_selector(
    ui: &mut Ui,
    integration: &RefCell<Integration>,
    world: &World,
) -> Option<entity_store::Index<Integrator>> {
    let integration = integration.borrow();
    let current_integrator_idx = integration.integrator_idx();
    let current_integrator = &world[current_integrator_idx];
    let mut selected_integrator_idx = current_integrator_idx;

    let integration_ptr: *const Integration = &*integration;
    egui::ComboBox::from_id_source(
        ui.make_persistent_id(format!("integrator_selector_{:?}", integration_ptr)),
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
    integration: &RefCell<Integration>,
    world: &World,
) -> Option<entity_store::Index<StepSize>> {
    let integration = integration.borrow();
    let integration_step_size_idx = integration.step_size_idx();
    let mut selected_step_size_idx = integration_step_size_idx;
    let integration_ptr: *const Integration = &*integration;
    egui::ComboBox::from_id_source(
        ui.make_persistent_id(format!("step_size_selector_{:?}", integration_ptr)),
    )
    .selected_text(format!("{}", world[integration_step_size_idx].borrow()))
    .show_ui(ui, |ui| {
        world
            .step_sizes()
            .enumerate()
            .for_each(|(each_idx, each_step_size)| {
                ui.selectable_value(
                    &mut selected_step_size_idx,
                    each_idx,
                    format!("{}", each_step_size.borrow()),
                );
            });
    });

    if selected_step_size_idx == integration_step_size_idx {
        None
    } else {
        Some(selected_step_size_idx)
    }
}
