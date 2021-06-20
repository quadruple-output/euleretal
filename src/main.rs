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
    if let Err(e) = flexi_logger::Logger::with_env_or_str("info")
        .format(flexi_logger::colored_opt_format)
        .start()
    {
        println!("Warning: Cannot initialize logging. {}", e);
    }

    let app = euleretal::Euleretal::default();
    eframe::run_native(Box::new(app), eframe::epi::NativeOptions::default());
}
