use super::{
    core::{self, Obj, Position, Scenario, Step},
    misc::{BoundingBox, Settings},
    ui_import::{Color32, Stroke},
    Integrator, StepSize,
};
use ::std::rc::Rc;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Integration {
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub core_integration: self::core::Integration,
    pub integrator: Obj<Integrator>,
    pub step_size: Obj<StepSize>,
    current_sample_index: Option<usize>,
}

impl ::std::fmt::Debug for Integration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Integration")
            //.field("core_integration", &self.core_integration)
            .field("integrator", &self.integrator)
            .field("step_size", &self.step_size)
            .field("current_sample_index", &self.current_sample_index)
            .finish()
    }
}

impl Clone for Integration {
    fn clone(&self) -> Self {
        Self::new(Rc::clone(&self.integrator), Rc::clone(&self.step_size))
    }
}

impl Integration {
    pub fn new(integrator: Obj<Integrator>, step_size: Obj<StepSize>) -> Self {
        Self {
            core_integration: self::core::Integration::new(),
            integrator,
            step_size,
            current_sample_index: None,
        }
    }

    pub fn reset(&mut self) {
        self.core_integration = self::core::Integration::new();
    }

    pub fn set_integrator(&mut self, integrator: Obj<Integrator>) {
        self.integrator = integrator;
        self.reset();
    }

    pub fn set_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_size = step_size;
        self.reset();
    }

    pub fn get_step_color(&self) -> Color32 {
        self.step_size.borrow().color
    }

    pub fn get_stroke(&self) -> Stroke {
        self.integrator.borrow().stroke
    }

    pub fn stretch_bbox(&self, bbox: &mut BoundingBox) {
        let integration = &self.core_integration;
        for samples in integration
            .reference_samples()
            .iter()
            .chain(integration.samples().iter())
        {
            samples
                .step_positions()
                .for_each(|position| bbox.expand_to(position));
        }
    }

    pub fn focus_closest_sample(&mut self, pos: &Position) {
        self.current_sample_index = self.core_integration.closest_sample_index(pos);
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn focussed_sample(&self) -> Option<(&Step, &Step)> {
        self.current_sample_index.map(|idx| {
            (
                self.core_integration.reference_samples().unwrap().at(idx),
                self.core_integration.samples().unwrap().at(idx),
            )
        })
    }

    pub fn update(&mut self, scenario: &Scenario) -> bool {
        if self.core_integration.update(
            scenario,
            &*self.integrator.borrow().integrator,
            self.step_size.borrow().duration,
        ) {
            self.adjust_focussed_sample();
            true
        } else {
            false
        }
    }

    fn adjust_focussed_sample(&mut self) {
        if let Some(prev_sample_idx) = self.current_sample_index {
            if let Some(samples) = self.core_integration.samples() {
                let num_samples = samples.len();
                if prev_sample_idx >= num_samples {
                    if num_samples > 0 {
                        self.current_sample_index = Some(num_samples - 1);
                    } else {
                        self.current_sample_index = None;
                    }
                }
            } else {
                self.current_sample_index = None;
            }
        }
    }

    pub fn draw_on(&self, canvas: &super::CanvasPainter, settings: &Settings) {
        let sample_color = self.step_size.borrow().color;
        let stroke = self.integrator.borrow().stroke;
        if let Some(samples) = self.core_integration.samples() {
            canvas.draw_sample_trajectory(samples, stroke);
        }
        if let Some(ref_samples) = self.core_integration.reference_samples() {
            canvas.draw_sample_dots(
                ref_samples,
                sample_color,
                &settings.point_formats.reference_position,
            );
        }
        if let Some(samples) = self.core_integration.samples() {
            canvas.draw_sample_dots(
                samples,
                sample_color,
                &settings.point_formats.derived_position,
            );
        }
    }
}
