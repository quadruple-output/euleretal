use crate::prelude::*;
use bevy_ecs::Entity;
use egui::Ui;

use super::{layers, BUTTON_GLYPH_ADD, BUTTON_GLYPH_DELETE};

enum Operation {
    Noop,
    Create,
    Delete {
        integration_id: Entity,
    },
    SetIntegrator {
        integration_id: Entity,
        integrator_id: Entity,
    },
    SetStepSize {
        integration_id: Entity,
        step_size_id: Entity,
    },
}

pub fn show(
    ui: &mut Ui,
    canvas_id: Entity,
    world: &mut World,
    size: Vec2,
    control_state: &ControlState,
) {
    ui.vertical(|ui| {
        let response = ui.horizontal(|ui| {
            show_scenario_selector(ui, canvas_id, world);
            show_integration_selector(ui, canvas_id, world);
        });

        let inner_size = Vec2::new(size.x, size.y - response.response.rect.height());
        let mut canvas = world.get_mut::<canvas::comp::State>(canvas_id).unwrap();
        let (response, painter) = canvas.allocate_painter(ui, inner_size);

        if control_state.layerflags.coordinates {
            layers::coordinates::render(world, &control_state, canvas_id, &response.rect, &painter);
        }
        if control_state.layerflags.acceleration_field {
            layers::acceleration_field::render(
                world,
                control_state,
                canvas_id,
                &response,
                &painter,
            );
        }
        layers::integrations::render(world, &control_state, canvas_id, &painter);
        if control_state.layerflags.inspector {
            layers::inspector::render(world, &control_state, canvas_id, &response, &painter);
        }
    });
}

fn show_scenario_selector(ui: &mut Ui, canvas_id: Entity, world: &mut World) {
    let selectable_scenarios: Vec<scenario::Gathered> = world
        .query::<scenario::Query>()
        .map(|scenario| scenario.gather_from(world))
        .collect();
    let canvas_scenario_id = world.get::<canvas::comp::ScenarioId>(canvas_id).unwrap().0;
    let canvas_scenario = selectable_scenarios
        .iter()
        .find(|scenario| scenario.id == canvas_scenario_id)
        .unwrap();

    let mut selected_scenario_id = canvas_scenario.id;
    egui::combo_box(
        ui,
        ui.make_persistent_id(format!("scenario_selector_{:?}", canvas_id)),
        canvas_scenario.label(),
        |ui| {
            for selectable_scenario in &selectable_scenarios {
                ui.selectable_value(
                    &mut selected_scenario_id,
                    selectable_scenario.id,
                    selectable_scenario.label(),
                );
            }
        },
    );
    if selected_scenario_id != canvas_scenario.id {
        let mut canvas_scenario_id = world
            .get_mut::<canvas::comp::ScenarioId>(canvas_id)
            .unwrap();
        *canvas_scenario_id = scenario::Entity(selected_scenario_id);
        let mut canvas_state = world.get_mut::<canvas::comp::State>(canvas_id).unwrap();
        canvas_state.reset_scenario();
    }
}

fn show_integration_selector(ui: &mut Ui, canvas_id: Entity, world: &mut World) {
    let mut window_is_open = world
        .get::<canvas::comp::State>(canvas_id)
        .unwrap()
        .ui_integrations_window_is_open;
    let button_response = ui.add(egui::Button::new("Integrations"));
    if button_response.clicked() {
        window_is_open = !window_is_open;
    };
    let operation = show_integrations_pop_up(
        ui,
        ui.make_persistent_id(format!("integrations_button_{:?}", canvas_id)),
        &mut window_is_open,
        Pos2::new(button_response.rect.left(), button_response.rect.bottom()),
        canvas_id,
        world,
    );
    world
        .get_mut::<canvas::comp::State>(canvas_id)
        .unwrap()
        .ui_integrations_window_is_open = window_is_open;

    match operation {
        Operation::Create => {
            let existing_integration = world
                .query::<integration::Query>()
                .map(|integration| integration.gather_from(world))
                .next()
                .unwrap();
            integration::Bundle(
                integration::Kind,
                integration::comp::State::new(integration::State::new()),
                existing_integration.integrator_id,
                existing_integration.step_size_id,
                canvas::Entity(canvas_id),
            )
            .spawn(world);
        }
        Operation::Delete { integration_id } => {
            world.despawn(integration_id).unwrap();
        }
        Operation::SetIntegrator {
            integration_id,
            integrator_id,
        } => {
            world
                .get_mut::<integration::comp::IntegratorId>(integration_id)
                .unwrap()
                .0 = integrator_id;
            world
                .get::<integration::comp::State>(integration_id)
                .unwrap()
                .lock()
                .unwrap()
                .reset();
        }
        Operation::SetStepSize {
            integration_id,
            step_size_id,
        } => {
            world
                .get_mut::<integration::comp::StepSizeId>(integration_id)
                .unwrap()
                .0 = step_size_id;
            world
                .get::<integration::comp::State>(integration_id)
                .unwrap()
                .lock()
                .unwrap()
                .reset();
        }
        Operation::Noop => (),
    }
}

fn show_integrations_pop_up(
    ui: &mut Ui,
    id: egui::Id,
    open: &mut bool,
    default_pos: Pos2,
    canvas_id: Entity,
    world: &World,
) -> Operation {
    let mut operation = Operation::Noop;

    let canvas_integrations: Vec<integration::Gathered> = world
        .query::<integration::Query>()
        .map(|integration| integration.gather_from(world))
        .filter(|integration| integration.canvas_id == canvas_id)
        .collect();

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
                    if ui.small_button(BUTTON_GLYPH_ADD).clicked() {
                        operation = Operation::Create;
                    }
                    ui.label("Integrator");
                    ui.label("Step Size");
                    ui.end_row();

                    // table body:
                    for integration in &canvas_integrations {
                        let integration_id = integration.id;
                        if canvas_integrations.len() > 1 {
                            if ui.small_button(BUTTON_GLYPH_DELETE).clicked() {
                                operation = Operation::Delete { integration_id };
                            }
                        } else {
                            ui.label("");
                        }
                        if let Some(integrator_id) =
                            show_integrator_selector(ui, integration, world)
                        {
                            operation = Operation::SetIntegrator {
                                integration_id,
                                integrator_id,
                            };
                        }
                        if let Some(step_size_id) = show_step_size_selector(ui, integration, world)
                        {
                            operation = Operation::SetStepSize {
                                integration_id,
                                step_size_id,
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
    integration: &integration::Gathered,
    world: &World,
) -> Option<Entity> {
    let selectable_integrators =
        world.query::<(Entity, &integrator::Kind, &integrator::comp::Integrator)>();
    let mut selected_integrator = integration.integrator_id.0;
    egui::combo_box(
        ui,
        ui.make_persistent_id(format!("integrator_selector_{:?}", integration.id)),
        integration.integrator.label(),
        |ui| {
            for selectable_integrator in selectable_integrators {
                ui.selectable_value(
                    &mut selected_integrator,
                    selectable_integrator.0,
                    selectable_integrator.2.label(),
                );
            }
        },
    );
    if selected_integrator == integration.integrator_id.0 {
        None
    } else {
        Some(selected_integrator)
    }
}

fn show_step_size_selector(
    ui: &mut Ui,
    integration: &integration::Gathered,
    world: &World,
) -> Option<Entity> {
    let mut selected_step_size_id = integration.step_size_id.0;
    egui::combo_box(
        ui,
        ui.make_persistent_id(format!("step_size_selector_{:?}", integration.id)),
        format!("{}", integration.step_size_id.gather_from(world)),
        |ui| {
            for selectable_step_size in world
                .query::<step_size::Query>()
                .map(|step_size| step_size.gather_from(world))
            {
                ui.selectable_value(
                    &mut selected_step_size_id,
                    selectable_step_size.id,
                    format!("{}", selectable_step_size),
                );
            }
        },
    );
    if selected_step_size_id == integration.step_size_id.0 {
        None
    } else {
        Some(selected_step_size_id)
    }
}
