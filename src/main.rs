/*
  bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
*/

#![feature(iterator_fold_self)]

mod acceleration;
mod canvas;
mod integrators;
mod layers;
mod sample;
mod scenarios;
mod ui;
mod view;

use acceleration::Acceleration;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use canvas::{Canvas, CanvasId};
use integrators::{ImplicitEuler, IntegrationParameters, IntegratorId};
use sample::Sample;
use scenarios::{center_mass::CenterMass, Scenario, ScenarioId};
use ui::UIState;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(EguiPlugin)
        .add_system(update_ui_scale_factor.system())
        .add_plugin(ui::Plugin)
        .add_plugin(layers::coordinates::Plugin)
        .add_plugin(layers::acceleration_field::Plugin)
        .add_plugin(layers::exact_path::Plugin)
        .add_plugin(layers::inspector::Plugin)
        .add_startup_system(initialize_scenario.system())
        .run();
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn initialize_scenario(commands: &mut Commands) {
    let mut scenario = Scenario::new(
        Box::new(CenterMass),
        Vec3::new(0., 0.8, 0.),
        Vec3::new(1.4, 0.3, 0.),
    );
    let parameters = IntegrationParameters {
        step_duration: 1.5,
        num_steps: 14,
    };
    let bbox = scenario.sample_bounding_box(&parameters);
    let mut canvas = Canvas::default();
    canvas
        .set_focus(bbox.center())
        .set_visible_units(bbox.diameter() * 1.2);
    let scenario_id = ScenarioId(commands.spawn((scenario,)).current_entity().unwrap());
    let canvas_id = CanvasId(commands.spawn((canvas,)).current_entity().unwrap());
    let integrator_id = IntegratorId(commands.spawn((ImplicitEuler,)).current_entity().unwrap());
    commands.spawn(view::IntegrationViewBundle {
        scenario_id,
        integrator_id,
        parameters,
        ui_state: Default::default(),
        canvas_id,
    });
}
