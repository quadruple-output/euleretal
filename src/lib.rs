//#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::non_ascii_literal)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
//#![deny(missing_docs)]
#![feature(box_syntax)]

#[macro_use]
mod core; // modules with macros must be listed first
mod integrators;
mod scenarios;
mod ui;

mod import {
    pub use ::bevy_math::Vec3;
    pub use ::decorum::R32;
    pub use ::std::rc::Rc;
}

mod ui_import {
    pub use ::eframe::{egui, epi};
    pub use egui::{
        color::{Hsva, Rgba},
        Color32, Pos2, Stroke, Ui, Vec2,
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
