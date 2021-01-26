/*
  bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
*/

mod acceleration;
mod canvas;
mod layers;
mod scenarios;
mod ui;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use scenarios::{center_mass::CenterMass, Scenario};

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
        .add_startup_system(initialize_scenario.system())
        .run();
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn initialize_scenario(commands: &mut Commands) {
    commands.spawn((Scenario {
        acceleration: Box::new(CenterMass),
    },));
}
