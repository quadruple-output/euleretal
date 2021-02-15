/*
  bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
*/

mod acceleration;
mod canvas;
mod functional_state;
mod integration;
mod integrators;
mod layers;
mod sample;
mod scenarios;
mod step_size;
mod ui;

use acceleration::Acceleration;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use canvas::Canvas;
use egui::{color::Hsva, Color32, Stroke};
use integration::Integration;
use integrators::{ConfiguredIntegrator, ImplicitEuler};
use sample::Sample;
use scenarios::{CenterMass, Scenario};
use std::f32::consts::TAU;
use step_size::StepSize;
use ui::UiState;

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
        .add_plugin(layers::integration::Plugin)
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
    let scenario = Scenario::new(
        Box::new(CenterMass),
        Vec3::new(0., 0.8, 0.),
        Vec3::new(1.4, 0.3, 0.),
        TAU,
    );
    let integrator = ConfiguredIntegrator {
        integrator: Box::new(ImplicitEuler),
        stroke: Stroke::new(1., Hsva::from(Color32::RED)),
    };
    let step_size = StepSize::new("long", 0.5, Hsva::from(Color32::YELLOW));
    let scenario_id = commands.spawn((scenario,)).current_entity().unwrap();
    let canvas = Canvas::new(scenario_id);
    let canvas_id = commands.spawn((canvas,)).current_entity().unwrap();
    let integrator_id = commands.spawn((integrator,)).current_entity().unwrap();
    let step_size_id = commands.spawn((step_size,)).current_entity().unwrap();
    let integration = Integration::new(step_size_id, canvas_id, integrator_id);
    commands.spawn((integration,));
}
