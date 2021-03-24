//#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
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
mod scenarios;
pub mod ui;

mod prelude {
    pub use super::components::prelude::*;
    pub use super::core::prelude::*;
    pub use super::entities::prelude::*;
    pub use super::misc::prelude::*;
    pub use super::ui::ControlState;
    pub use bevy_ecs::World;
    pub use bevy_math::Vec3;
    pub use decorum::R32;
    pub use eframe::egui;
    pub use eframe::egui::{color::Hsva, Color32, Pos2, Stroke, Ui, Vec2};
}

use self::prelude::*;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = ui::App::default();
    eframe::start_web(canvas_id, Box::new(app))
}
