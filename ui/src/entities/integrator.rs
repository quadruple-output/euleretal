#[cfg(feature = "persistence")]
use super::core;

#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct Integrator {
    #[cfg_attr(
        feature = "persistence",
        serde(with = "core::integrators::serde_box_dyn_integrator")
    )]
    pub integrator: Box<dyn crate::core::Integrator>,
    pub stroke: eframe::egui::Stroke,
}
