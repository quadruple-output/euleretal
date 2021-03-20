#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = euleretal::TemplateApp::default();
    eframe::run_native(Box::new(app));
}
