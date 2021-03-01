// https://github.com/rust-lang/rust-clippy
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod components;
mod core;
mod entities;
mod integrators;
mod misc;
mod plugins;
mod scenarios;

mod prelude {
    pub use crate::components::prelude::*;
    pub use crate::core::prelude::*;
    pub use crate::entities::prelude::*;
    pub use crate::misc::prelude::*;
    pub use crate::plugins::ui::State as UiState; // TODO: THIS IS A RESOURCE AND SHOULD NOT BE IN PLUGINS
    pub use ::bevy::math::Vec3;
    pub use ::decorum::R32;
    pub use ::egui::{color::Hsva, Color32, Pos2, Stroke, Ui, Vec2};
}

use crate::prelude::*;
use ::bevy::prelude::*;

fn main() {
    if let Err(e) = flexi_logger::Logger::with_env_or_str("info")
        .format(flexi_logger::colored_opt_format)
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
        .add_system(::bevy::input::system::exit_on_esc_system.system())
        .add_plugin(::bevy_egui::EguiPlugin)
        .add_system(update_ui_scale_factor.system())
        .add_plugin(plugins::Ui)
        .add_plugin(plugins::Coordinates)
        .add_plugin(plugins::AccelerationField)
        .add_plugin(plugins::Integrations)
        .add_plugin(plugins::Inspector)
        .add_startup_system(initialize_scenario.system())
        .run();
}

#[allow(clippy::needless_pass_by_value)]
fn update_ui_scale_factor(
    mut egui_settings: ResMut<bevy_egui::EguiSettings>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn initialize_scenario(commands: &mut Commands) {
    let step_size_id = step_size::Bundle(
        step_size::Kind,
        UserLabel("long".to_string()),
        Duration(ChangeTracker::with(R32::from(0.5))),
        Hsva::from(Color32::YELLOW),
    )
    .spawn(commands);

    let integrator_id = integrator::Bundle(
        integrator::Kind,
        Box::new(integrators::euler::Implicit),
        Stroke::new(1., Hsva::from(Color32::RED)),
    )
    .spawn(commands);

    let scenario_center_mass_id = scenario::Bundle(
        scenario::Kind,
        Box::new(scenarios::CenterMass),
        StartPosition(ChangeTracker::with(Vec3::new(0., 1., 0.))),
        StartVelocity(ChangeTracker::with(Vec3::new(1., 0., 0.))),
        Duration(ChangeTracker::with(::std::f32::consts::TAU.into())),
    )
    .spawn(commands);

    let scenario_constant_acceleration_id = scenario::Bundle(
        scenario::Kind,
        Box::new(scenarios::ConstantAcceleration),
        StartPosition(ChangeTracker::with(Vec3::new(0., 0., 0.))),
        StartVelocity(ChangeTracker::with(Vec3::new(1., 0., 0.))),
        Duration(ChangeTracker::with(2_f32.into())),
    )
    .spawn(commands);

    let canvas_center_mass_id =
        canvas::Bundle(canvas::Kind, canvas::State::new(), scenario_center_mass_id).spawn(commands);

    let canvas_constant_acceleration_id = canvas::Bundle(
        canvas::Kind,
        canvas::comp::State::new(),
        scenario_constant_acceleration_id,
    )
    .spawn(commands);

    integration::Bundle(
        integration::Kind,
        integration::comp::State::new(),
        integrator_id,
        step_size_id,
        canvas_center_mass_id,
    )
    .spawn(commands);

    integration::Bundle(
        integration::Kind,
        integration::comp::State::new(),
        integrator_id,
        step_size_id,
        canvas_constant_acceleration_id,
    )
    .spawn(commands);
}
