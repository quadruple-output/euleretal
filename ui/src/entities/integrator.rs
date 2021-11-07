#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Integrator {
    pub integrator: Box<dyn crate::core::Integrator>,
    pub stroke: eframe::egui::Stroke,
}
