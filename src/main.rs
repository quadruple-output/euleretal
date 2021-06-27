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
    use ::log::LevelFilter;
    use ::simple_logger::SimpleLogger;

    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("euleretal", LevelFilter::Info)
        .init()
        .unwrap();

    let app = euleretal::Euleretal::default();
    eframe::run_native(Box::new(app), eframe::epi::NativeOptions::default());
}
