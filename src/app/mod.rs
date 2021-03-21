mod components;
mod core;
mod entities;
mod integrators;
mod misc;
mod scenarios;

mod prelude {
    pub use super::components::prelude::*;
    pub use super::core::prelude::*;
    pub use super::entities::prelude::*;
    pub use super::misc::prelude::*;
    pub use ::bevy_math::Vec3;
    pub use ::decorum::R32;
    pub use ::eframe::egui;
    pub use ::eframe::egui::{color::Hsva, Color32, Pos2, Stroke, Ui, Vec2};
}

use self::prelude::*;
use ::bevy_ecs::prelude::*;

use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct App {
    world: World,
    resources: Resources,
}

impl Default for App {
    fn default() -> Self {
        let mut default = Self::new();
        default.initialize_scenario();
        default
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "euleretal"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {}
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
        }
    }

    fn initialize_scenario(&mut self) {
        let mut commands = Commands::default();
        commands.set_entity_reserver(self.world.get_entity_reserver());
        let step_size_id = step_size::Bundle(
            step_size::Kind,
            UserLabel("long".to_string()),
            Duration(ChangeTracker::with(R32::from(0.5))),
            Hsva::from(Color32::YELLOW),
        )
        .spawn(&mut commands);

        let integrator_id = integrator::Bundle(
            integrator::Kind,
            Box::new(integrators::euler::Implicit),
            Stroke::new(1., Hsva::from(Color32::RED)),
        )
        .spawn(&mut commands);

        let scenario_center_mass_id = scenario::Bundle(
            scenario::Kind,
            Box::new(scenarios::CenterMass),
            StartPosition(ChangeTracker::with(Vec3::new(0., 1., 0.))),
            StartVelocity(ChangeTracker::with(Vec3::new(1., 0., 0.))),
            Duration(ChangeTracker::with(::std::f32::consts::TAU.into())),
        )
        .spawn(&mut commands);

        let scenario_constant_acceleration_id = scenario::Bundle(
            scenario::Kind,
            Box::new(scenarios::ConstantAcceleration),
            StartPosition(ChangeTracker::with(Vec3::new(0., 0., 0.))),
            StartVelocity(ChangeTracker::with(Vec3::new(1., 0., 0.))),
            Duration(ChangeTracker::with(2_f32.into())),
        )
        .spawn(&mut commands);

        let canvas_center_mass_id =
            canvas::Bundle(canvas::Kind, canvas::State::new(), scenario_center_mass_id)
                .spawn(&mut commands);

        let canvas_constant_acceleration_id = canvas::Bundle(
            canvas::Kind,
            canvas::comp::State::new(),
            scenario_constant_acceleration_id,
        )
        .spawn(&mut commands);

        integration::Bundle(
            integration::Kind,
            integration::comp::State::new(),
            integrator_id,
            step_size_id,
            canvas_center_mass_id,
        )
        .spawn(&mut commands);

        integration::Bundle(
            integration::Kind,
            integration::comp::State::new(),
            integrator_id,
            step_size_id,
            canvas_constant_acceleration_id,
        )
        .spawn(&mut commands);

        commands.apply(&mut self.world, &mut self.resources);
    }
}
