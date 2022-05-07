use super::{
    core::{self, Duration, Position, Scenario, Step},
    misc::BoundingBox,
    Integrator, StepSize, World,
};
use crate::misc::entity_store;

#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct Integration {
    #[serde(skip)]
    core: self::core::Integration,
    #[serde(rename = "integrator")]
    integrator_idx: entity_store::Index<Integrator>,
    #[serde(rename = "step_size")]
    step_size_idx: entity_store::Index<StepSize>,
    current_sample_index: Option<usize>,
}

impl ::std::fmt::Debug for Integration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Integration")
            //.field("core_integration", &self.core_integration)
            .field("integrator", &self.integrator_idx)
            .field("step_size", &self.step_size_idx)
            .field("current_sample_index", &self.current_sample_index)
            .finish()
    }
}

impl Clone for Integration {
    fn clone(&self) -> Self {
        Self::new(self.integrator_idx, self.step_size_idx)
    }
}

impl Integration {
    pub fn new(
        integrator: entity_store::Index<Integrator>,
        step_size: entity_store::Index<StepSize>,
    ) -> Self {
        Self {
            core: self::core::Integration::new(),
            integrator_idx: integrator,
            step_size_idx: step_size,
            current_sample_index: None,
        }
    }

    pub fn reset(&mut self) {
        self.core = self::core::Integration::new();
    }

    pub fn integrator_idx(&self) -> entity_store::Index<Integrator> {
        self.integrator_idx
    }

    pub fn step_size_idx(&self) -> entity_store::Index<StepSize> {
        self.step_size_idx
    }

    pub fn set_integrator(&mut self, integrator_idx: entity_store::Index<Integrator>) {
        self.integrator_idx = integrator_idx;
        self.reset();
    }

    pub fn set_step_size(&mut self, step_size_idx: entity_store::Index<StepSize>) {
        self.step_size_idx = step_size_idx;
        self.reset();
    }

    pub fn fetch_step_duration(&self, world: &World) -> Duration {
        world[self.step_size_idx].borrow().duration
    }

    pub fn stretch_bbox(&self, bbox: &mut BoundingBox) {
        let integration = &self.core;
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
        self.current_sample_index = self.core.closest_sample_index(pos);
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn focussed_sample(&self) -> Option<(&Step, &Step)> {
        self.current_sample_index.map(|idx| {
            (
                self.core.reference_samples().unwrap().at(idx), // todo : idx could be invalid (by loading from incompatible save file)
                self.core.samples().unwrap().at(idx),
            )
        })
    }

    pub fn update(
        &mut self,
        scenario: &Scenario,
        integrator: &dyn core::Integrator,
        step_duration: Duration,
    ) -> bool {
        if self.core.update(scenario, integrator, step_duration) {
            self.adjust_focussed_sample();
            true
        } else {
            false
        }
    }

    fn adjust_focussed_sample(&mut self) {
        if let Some(prev_sample_idx) = self.current_sample_index {
            if let Some(samples) = self.core.samples() {
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

    pub fn draw_on(&self, canvas: &super::CanvasPainter, world: &World) {
        let sample_color = world[self.step_size_idx].borrow().color;
        if let Some(reference_samples) = self.core.reference_samples() {
            canvas.draw_sample_dots(
                reference_samples,
                sample_color,
                &world.settings.point_formats.reference_position,
            );
        }
        if let Some(samples) = self.core.samples() {
            canvas.draw_sample_trajectory(samples, world[self.integrator_idx].borrow().stroke);
            canvas.draw_sample_dots(
                samples,
                sample_color,
                &world.settings.point_formats.derived_position,
            );
        }
    }

    pub fn check_references(&self, world: &World) -> Result<(), String> {
        self.integrator_idx
            .check_reference(world.integrators())
            .map_err(|err| format!("integrator: {}", err))?;
        self.step_size_idx
            .check_reference(world.step_sizes())
            .map_err(|err| format!("step size: {}", err))?;
        Ok(())
    }
}
