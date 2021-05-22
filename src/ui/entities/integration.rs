use super::{Integrator, StepSize};
use crate::prelude::*;

pub struct Integration {
    pub core_integration: crate::core::Integration,
    pub integrator: Obj<Integrator>,
    pub step_size: Obj<StepSize>,
}

impl Clone for Integration {
    fn clone(&self) -> Self {
        Self::new(Rc::clone(&self.integrator), Rc::clone(&self.step_size))
    }
}

impl Integration {
    pub fn new(integrator: Obj<Integrator>, step_size: Obj<StepSize>) -> Self {
        Self {
            core_integration: crate::core::Integration::new(),
            integrator,
            step_size,
        }
    }

    pub fn reset(&mut self) {
        self.core_integration = crate::core::Integration::new();
    }

    pub fn set_integrator(&mut self, integrator: Obj<Integrator>) {
        self.integrator = integrator;
        self.reset();
    }

    pub fn set_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_size = step_size;
        self.reset();
    }

    pub fn get_step_color(&self) -> Hsva {
        self.step_size.borrow().color
    }

    pub fn get_stroke(&self) -> Stroke {
        self.integrator.borrow().stroke
    }

    pub fn stretch_bbox(&self, bbox: &mut crate::ui::BoundingBox) {
        let integration = &self.core_integration;
        for samples in integration
            .reference_samples()
            .iter()
            .chain(integration.samples().iter())
        {
            samples
                .step_points()
                .iter()
                .for_each(|&point| bbox.expand_to(point));
        }
    }

    pub fn closest_sample(&self, pos: Vec3) -> Option<(CompleteSample, CompleteSample)> {
        self.core_integration.closest_sample(pos)
    }

    pub fn update(&mut self, scenario: &Scenario) {
        self.core_integration.update(
            scenario,
            &*self.integrator.borrow().integrator,
            self.step_size.borrow().duration,
        )
    }

    pub fn draw_on(&self, canvas: &super::Canvas, painter: &egui::Painter) {
        let sample_color = Color32::from(self.step_size.borrow().color);
        let stroke = self.integrator.borrow().stroke;
        if let Some(ref samples) = self.core_integration.samples() {
            canvas.draw_sample_trajectory(&samples, stroke, painter);
        }
        for samples in self
            .core_integration
            .reference_samples()
            .iter()
            .chain(self.core_integration.samples().iter())
        {
            canvas.draw_sample_dots(samples, sample_color, painter);
        }
    }
}