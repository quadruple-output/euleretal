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
    // Example stuff:
    value: f32,
}

impl Default for App {
    fn default() -> Self {
        Self { value: 2.7 }
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
    fn update(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.value *= 1.0;
    }
}
