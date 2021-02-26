// https://github.com/rust-lang/rust-clippy
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod acceleration;
mod bounding_box;
mod canvas;
mod change_tracker;
mod integration;
mod integrator;
mod layer;
mod sample;
mod scenario;
mod step_size;
mod ui;

use acceleration::Acceleration;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bounding_box::BoundingBox;
use canvas::Canvas;
use change_tracker::{ChangeCount, ChangeTracker, TrackedChange};
use egui::{color::Hsva, Color32, Stroke};
use flexi_logger::{colored_opt_format, Logger};
use integration::Integration;
use integrator::euler::Implicit as ImplicitEuler;
use sample::Sample;
use scenario::{CenterMass, ConstantAcceleration, Scenario};
use std::f32::consts::TAU;
use step_size::StepSize;
use ui::State as UiState;

fn main() {
    if let Err(e) = Logger::with_env_or_str("info")
        .format(colored_opt_format)
        .start()
    {
        println!("Warning: Cannot initialize logging. {}", e);
    }
    /*
      bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
    */
    App::build()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(EguiPlugin)
        .add_system(update_ui_scale_factor.system())
        .add_plugin(ui::Plugin)
        .add_plugin(layer::coordinates::Plugin)
        .add_plugin(layer::acceleration_field::Plugin)
        .add_plugin(layer::integrations::Plugin)
        .add_plugin(layer::inspector::Plugin)
        .add_startup_system(initialize_scenario.system())
        .run();
}

#[allow(clippy::needless_pass_by_value)]
fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn initialize_scenario(commands: &mut Commands) {
    let step_size = StepSize::new("long", 0.5.into(), Hsva::from(Color32::YELLOW));
    let step_size_id = step_size::Entity(commands.spawn((step_size,)).current_entity().unwrap());

    let integrator_id = integrator::Entity(
        commands
            .spawn(integrator::Bundle {
                integrator: Box::new(ImplicitEuler),
                stroke: Stroke::new(1., Hsva::from(Color32::RED)),
            })
            .current_entity()
            .unwrap(),
    );

    let scenario_center_mass = Scenario::new(
        Box::new(CenterMass),
        Vec3::new(0., 1., 0.),
        Vec3::new(1., 0., 0.),
        TAU.into(),
    );
    let scenario_center_mass_id = scenario::Entity(
        commands
            .spawn((scenario_center_mass,))
            .current_entity()
            .unwrap(),
    );

    let scenario_constant_acceleration = Scenario::new(
        Box::new(ConstantAcceleration),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        2_f32.into(),
    );
    let scenario_constant_acceleration_id = scenario::Entity(
        commands
            .spawn((scenario_constant_acceleration,))
            .current_entity()
            .unwrap(),
    );

    let canvas_center_mass_id = canvas::Entity(
        commands
            .spawn((Canvas::new(), scenario_center_mass_id))
            .current_entity()
            .unwrap(),
    );

    let canvas_constant_acceleration_id = canvas::Entity(
        commands
            .spawn((Canvas::new(), scenario_constant_acceleration_id))
            .current_entity()
            .unwrap(),
    );

    commands.spawn((
        Integration::new(),
        step_size_id,
        canvas_center_mass_id,
        integrator_id,
    ));

    commands.spawn((
        Integration::new(),
        step_size_id,
        canvas_constant_acceleration_id,
        integrator_id,
    ));
}
