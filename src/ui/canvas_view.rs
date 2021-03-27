use crate::prelude::*;
use egui::Ui;

use super::layers;

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
    let button_response = ui.add(egui::Button::new("Integrations"));
    let mut canvas_state = unsafe {
        world
            .get_mut_unchecked::<canvas::comp::State>(canvas_id)
            .unwrap()
    };
    if button_response.clicked() {
        canvas_state.ui_integrations_window_open = !canvas_state.ui_integrations_window_open;
    };

    egui::Window::new("Integrations")
        .id(ui.make_persistent_id(format!("integrations_button_{:?}", canvas_id)))
        .open(&mut canvas_state.ui_integrations_window_open)
        .collapsible(false)
        .default_pos(button_response.rect.min)
        .show(ui.ctx(), |ui| {
            let canvas_integrations: Vec<integration::Gathered> = world
                .query::<integration::Query>()
                .map(|integration| integration.gather_from(world))
                .filter(|integration| integration.canvas_id == canvas_id)
                .collect();
            egui::Grid::new("integrator grid")
                .striped(true)
                .show(ui, |ui| {
                    for integration in &canvas_integrations {
                        ui.label(integration.integrator.label());
                        ui.label(format!(
                            "{} ({})",
                            integration.step_label,
                            integration.step_duration.get()
                        ));
                        if canvas_integrations.len() > 1 {
                            ui.small_button("\u{2796}"); // \u{2796}='➖', \u{1fsd1} = '🗑'
                        }
                        ui.end_row();
                    }
                });
            if ui.button("\u{271a}").clicked() { // ✚
                 //world.spawn(integration::Bundle::from(()));
            }
        });
}
