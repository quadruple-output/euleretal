pub struct Integrator {
    pub integrator: Box<dyn crate::core::Integrator>,
    pub stroke: eframe::egui::Stroke,
}
