use crate::prelude::*;

pub struct ConfiguredIntegrator {
    pub integrator: Box<dyn Integrator>,
    pub stroke: eframe::egui::Stroke,
}
