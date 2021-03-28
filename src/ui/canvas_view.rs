use crate::prelude::*;
use egui::Ui;

use super::{layers, BUTTON_GLYPH_ADD, BUTTON_GLYPH_DELETE};

pub fn show(
    ui: &mut Ui,
    canvas_id: bevy_ecs::Entity,
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

fn show_scenario_selector(ui: &mut Ui, canvas_id: bevy_ecs::Entity, world: &mut World) {
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

fn show_integration_selector(ui: &mut Ui, canvas_id: bevy_ecs::Entity, world: &mut World) {
    let operation;
    {
        // block makes sure that unsafely acquired canvas_state is not used longer
        // than expected
        let button_response = ui.add(egui::Button::new("Integrations"));
        let mut canvas_state = unsafe {
            world
                .get_mut_unchecked::<canvas::comp::State>(canvas_id)
                .unwrap()
        };
        if button_response.clicked() {
            canvas_state.ui_integrations_window_open = !canvas_state.ui_integrations_window_open;
        };
        operation = show_integrations_pop_up(
            ui,
            ui.make_persistent_id(format!("integrations_button_{:?}", canvas_id)),
            &mut canvas_state.ui_integrations_window_open,
            Pos2::new(button_response.rect.left(), button_response.rect.bottom()),
            canvas_id,
            world,
        );
    }

    #[allow(clippy::single_match, clippy::match_bool)]
    {
        match operation.create {
            true => {
                let best_integration = world
                    .query::<integration::Query>()
                    .map(|integration| integration.gather_from(world))
                    .next()
                    .unwrap();
                integration::Bundle(
                    integration::Kind,
                    integration::comp::State::new(integration::State::new()),
                    best_integration.integrator_id,
                    best_integration.step_size_id,
                    canvas::Entity(canvas_id),
                )
                .spawn(world);
            }
            false => (),
        }
        match operation.delete {
            Some(integration_id) => {
                world.despawn(integration_id).unwrap();
            }
            None => (),
        }
        match operation.swap_integrator {
            Some((integration_id, integrator_id)) => {
                let mut integration_integrator_id = world
                    .get_mut::<integration::comp::IntegratorId>(integration_id.0)
                    .unwrap();
                *integration_integrator_id = integrator_id;
                let integration_state = world
                    .get::<integration::comp::State>(integration_id.0)
                    .unwrap();
                integration_state.lock().unwrap().reset();
            }
            None => (),
        }
        match operation.swap_step_size {
            Some((integration_id, step_size_id)) => {
                let mut integration_step_size_id = world
                    .get_mut::<integration::comp::StepSizeId>(integration_id.0)
                    .unwrap();
                *integration_step_size_id = step_size_id;
                let integration_state = world
                    .get::<integration::comp::State>(integration_id.0)
                    .unwrap();
                integration_state.lock().unwrap().reset();
            }
            None => (),
        }
    }
}

#[derive(Default)]
struct IntegrationsOperation {
    create: bool,
    delete: Option<bevy_ecs::Entity>,
    swap_integrator: Option<(integration::Entity, integrator::Entity)>,
    swap_step_size: Option<(integration::Entity, step_size::Entity)>,
}

fn show_integrations_pop_up(
    ui: &mut Ui,
    id: egui::Id,
    open: &mut bool,
    default_pos: Pos2,
    canvas_id: bevy_ecs::Entity,
    world: &bevy_ecs::World,
) -> IntegrationsOperation {
    let mut operation = IntegrationsOperation::default();

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
                        operation.create = true;
                    }
                    ui.label("Integrator");
                    ui.label("Step Size");
                    ui.end_row();

                    // table body:
                    for integration in &canvas_integrations {
                        if canvas_integrations.len() > 1 {
                            if ui.small_button(BUTTON_GLYPH_DELETE).clicked() {
                                operation.delete = Some(integration.id);
                            }
                        } else {
                            ui.label("");
                        }
                        operation.swap_integrator =
                            show_integrator_selector(ui, integration, world);
                        operation.swap_step_size = show_step_size_selector(ui, integration, world);
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
) -> Option<(integration::Entity, integrator::Entity)> {
    let selectable_integrators = world.query::<(
        bevy_ecs::Entity,
        &integrator::Kind,
        &integrator::comp::Integrator,
    )>();
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
        Some((
            integration::Entity(integration.id),
            integrator::Entity(selected_integrator),
        ))
    }
}

fn show_step_size_selector(
    ui: &mut Ui,
    integration: &integration::Gathered,
    world: &World,
) -> Option<(integration::Entity, step_size::Entity)> {
    let selectable_step_sizes = world.query::<(
        bevy_ecs::Entity,
        &step_size::Kind,
        &step_size::comp::Duration,
        &step_size::comp::UserLabel,
    )>();
    let mut selected_step_size = integration.step_size_id.0;
    egui::combo_box(
        ui,
        ui.make_persistent_id(format!("step_size_selector_{:?}", integration.id)),
        format!(
            "\"{}\" ({})",
            integration.step_label,
            integration.step_duration.get()
        ),
        |ui| {
            for selectable_step_size in selectable_step_sizes {
                ui.selectable_value(
                    &mut selected_step_size,
                    selectable_step_size.0,
                    format!(
                        "\"{}\" ({})",
                        selectable_step_size.3 .0,
                        selectable_step_size.2 .0.get()
                    ),
                );
            }
        },
    );
    if selected_step_size == integration.step_size_id.0 {
        None
    } else {
        Some((
            integration::Entity(integration.id),
            step_size::Entity(selected_step_size),
        ))
    }
}
