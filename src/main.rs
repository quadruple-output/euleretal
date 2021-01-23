/*
  bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
*/

mod ui;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};

fn main() {
    App::build()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(EguiPlugin)
        .add_system(update_ui_scale_factor.system())
        .add_plugin(ui::Plugin)
        .run();
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}
