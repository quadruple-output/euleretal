use crate::{Canvas, ChangeCount, ConfiguredIntegrator, Sample, Scenario, StepSize, TrackedChange};
use bevy::prelude::*;
use egui::{Color32, Stroke};

pub struct Integration {
    step_size_id: Entity,
    canvas_id: Entity,
    integrator_id: Entity,
    samples: Vec<Sample>,
    samples_change_count: ChangeCount,
    reference_samples: Vec<Sample>,
    ref_samples_change_count: ChangeCount,
}

impl Integration {
    pub fn new(step_size_id: Entity, canvas_id: Entity, integrator_id: Entity) -> Self {
        Self {
            step_size_id,
            canvas_id,
            integrator_id,
            samples: Default::default(),
            samples_change_count: Default::default(),
            reference_samples: Default::default(),
            ref_samples_change_count: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        scenario: &Scenario,
        integrator: &ConfiguredIntegrator,
        step_size: &StepSize,
    ) {
        let ref_samples_change_count = step_size.change_count() + scenario.change_count();
        let samples_change_count = ref_samples_change_count + integrator.change_count();
        if self.samples_change_count != samples_change_count {
            self.samples = integrator.integrate(&scenario, step_size.dt.get());
            self.samples_change_count = samples_change_count;
            if self.ref_samples_change_count != ref_samples_change_count {
                self.reference_samples = scenario.calculate_reference_samples(step_size.dt.get());
                self.ref_samples_change_count = ref_samples_change_count;
            }
        }
    }

    pub fn get_canvas_id(&self) -> Entity {
        self.canvas_id
    }

    pub fn get_step_size<'a>(
        &self,
        query: &'a Query<&StepSize>,
    ) -> Result<&'a StepSize, bevy::ecs::QueryError> {
        query.get(self.step_size_id)
    }

    pub fn get_integrator<'a>(
        &self,
        query: &'a Query<&ConfiguredIntegrator>,
    ) -> Result<&'a ConfiguredIntegrator, bevy::ecs::QueryError> {
        query.get(self.integrator_id)
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn closest_sample(&self, pos: Vec3) -> Option<(Sample, Sample)> {
        if self.reference_samples.is_empty() {
            None
        } else {
            let (closest_reference, dist_ref) = Self::find_closest(&self.reference_samples, pos);
            let (closest_sample, dist_sample) = Self::find_closest(&self.samples, pos);
            if dist_ref < dist_sample {
                Some((closest_reference, self.samples[closest_reference.n]))
            } else {
                Some((self.reference_samples[closest_sample.n], closest_sample))
            }
        }
    }

    fn find_closest(samples: &[Sample], pos: Vec3) -> (Sample, f32) {
        assert!(!samples.is_empty());
        samples
            .iter()
            .map(|s| (s, (s.s - pos).length_squared()))
            .fold((Sample::default(), f32::MAX), |(s0, d0), (&s1, d1)| {
                if d0 < d1 {
                    (s0, d0)
                } else {
                    (s1, d1)
                }
            })
    }

    pub fn draw_on(&self, canvas: &mut Canvas, sample_color: Color32, stroke: Stroke) {
        canvas.draw_sample_trajectory(&self.samples, stroke);
        canvas.draw_sample_dots(&self.reference_samples, sample_color);
        canvas.draw_sample_dots(&self.samples, sample_color);
    }
}
