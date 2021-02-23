/*
  bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
*/

mod acceleration;
mod canvas;
mod change_tracker;
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
use change_tracker::*;
use egui::{color::Hsva, Color32, Stroke};
use flexi_logger::{colored_opt_format, Logger};
use integration::Integration;
use integrators::{ConfiguredIntegrator, ImplicitEuler};
use sample::Sample;
use scenarios::{CenterMass, ConstantAcceleration, Scenario};
use std::f32::consts::TAU;
use step_size::StepSize;
use ui::UiState;

fn main() {
    if let Err(e) = Logger::with_env_or_str("debug")
        .format(colored_opt_format)
        .start()
    {
        println!("Warning: Cannot initialize logging. {}", e);
    }
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
    let step_size = StepSize::new("long", 0.5, Hsva::from(Color32::YELLOW));
    let step_size_id = commands.spawn((step_size,)).current_entity().unwrap();

    let integrator = ConfiguredIntegrator {
        integrator: Box::new(ImplicitEuler),
        stroke: Stroke::new(1., Hsva::from(Color32::RED)),
    };
    let integrator_id = commands.spawn((integrator,)).current_entity().unwrap();

    let scenario_center_mass = Scenario::new(
        Box::new(CenterMass),
        Vec3::new(0., 1., 0.),
        Vec3::new(1., 0., 0.),
        TAU,
    );
    let scenario_id_center_mass = commands
        .spawn((scenario_center_mass,))
        .current_entity()
        .unwrap();

    let scenario_constant_acceleration = Scenario::new(
        Box::new(ConstantAcceleration),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        2.,
    );
    let scenario_id_constant_acceleration = commands
        .spawn((scenario_constant_acceleration,))
        .current_entity()
        .unwrap();

    let canvas_center_mass = Canvas::new(scenario_id_center_mass);
    let canvas_id_center_mass = commands
        .spawn((canvas_center_mass,))
        .current_entity()
        .unwrap();

    let canvas_constant_acceleration = Canvas::new(scenario_id_constant_acceleration);
    let canvas_id_constant_acceleration = commands
        .spawn((canvas_constant_acceleration,))
        .current_entity()
        .unwrap();

    let integration_center_mass =
        Integration::new(step_size_id, canvas_id_center_mass, integrator_id);
    commands.spawn((integration_center_mass,));

    let integration_constant_acceleration =
        Integration::new(step_size_id, canvas_id_constant_acceleration, integrator_id);
    commands.spawn((integration_constant_acceleration,));
}
