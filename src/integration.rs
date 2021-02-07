use crate::{Canvas, Sample};
use bevy::prelude::*;
use egui::{Color32, Stroke};

pub struct Integration {
    step_size_id: Entity,
    canvas_id: Entity,
    integrator_id: Entity,
    samples: Vec<Sample>,
    reference_samples: Vec<Sample>,
}

impl Integration {
    pub fn new(step_size_id: Entity, canvas_id: Entity, integrator_id: Entity) -> Self {
        Self {
            step_size_id,
            canvas_id,
            integrator_id,
            samples: Default::default(),
            reference_samples: Default::default(),
        }
    }

    pub fn set_integration_steps(&mut self, integration_steps: Vec<Sample>) {
        self.samples = integration_steps;
    }

    pub fn set_reference_samples(&mut self, reference_samples: Vec<Sample>) {
        self.reference_samples = reference_samples;
    }

    pub fn get_canvas_id(&self) -> Entity {
        self.canvas_id
    }

    pub fn get_step_size_id(&self) -> Entity {
        self.step_size_id
    }

    pub fn get_integrator_id(&self) -> Entity {
        self.integrator_id
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

    pub fn draw_on(&self, canvas: &Canvas, dot_color: Color32, stroke: Stroke) {
        self.reference_samples
            .iter()
            .for_each(|sample| canvas.dot(sample.s, dot_color));
        self.samples
            .iter()
            .for_each(|sample| canvas.dot(sample.s, dot_color));
        self.samples.iter().fold_first(|s0, s1| {
            canvas.line_segment(s0.s, s1.s, stroke);
            s1
        });
    }
}
