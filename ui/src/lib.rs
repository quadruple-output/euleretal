//#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::multiple_crate_versions)]
//#![deny(missing_docs)]

mod app;
mod containers;
mod entities;
mod misc;
mod world;

mod import {
    pub type Point3 = ::euleretal_core::Point3;
    pub type Vec3 = ::euleretal_core::Vec3;
}

mod ui_import {
    pub use ::eframe::{
        egui::{
            self,
            color::{Color32, Hsva, Rgba},
            emath::{Pos2, Vec2},
            epaint::Stroke,
            Ui,
        },
        epi,
    };
}

use ::euleretal_core as core;
pub use app::Euleretal;
use world::World;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use ::eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), ::eframe::wasm_bindgen::JsValue> {
    ::std::panic::set_hook(Box::new(::console_error_panic_hook::hook));
    let app = Euleretal::default();
    ::eframe::start_web(canvas_id, Box::new(app))
}
