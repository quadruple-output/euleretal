use super::core;

#[derive(Debug, ::serde::Deserialize, ::serde::Serialize)]
pub struct Integrator {
    #[serde(with = "core::integrators::serde_box_dyn_integrator")]
    pub core: Box<dyn crate::core::Integrator>,
    pub stroke: eframe::egui::Stroke,
}
