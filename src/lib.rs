//#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
//#![deny(missing_docs)]

mod core;
mod integrators;
mod scenarios;
mod ui;

mod import {
    pub use ::parry3d::{query::PointQuery, shape};
    pub type OrderedF32 = ::ordered_float::OrderedFloat<f32>;
    pub type Point3 = ::parry3d::math::Point<f32>;
    pub type Vec3 = ::parry3d::math::Vector<f32>;
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

pub use ui::Euleretal;

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
    let app = Euleretal::default();
    ::eframe::start_web(canvas_id, Box::new(app))
}
