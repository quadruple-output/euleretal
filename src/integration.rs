use crate::{Canvas, Sample};
use bevy::prelude::*;
use egui::Color32;

pub struct Integration {
    step_size_id: Entity,
    canvas_id: Entity,
    integrator_id: Entity,
    samples: Vec<Sample>,
    reference_samples: Vec<Sample>,
}

impl Integration {
    pub fn new(
        step_size_id: Entity,
        canvas_id: Entity,
        integrator_id: Entity,
        reference_samples: Vec<Sample>,
    ) -> Self {
        Self {
            step_size_id,
            canvas_id,
            integrator_id,
            samples: Default::default(),
            reference_samples,
        }
    }

    pub fn get_canvas_id(&self) -> Entity {
        self.canvas_id
    }

    pub fn get_step_size_id(&self) -> Entity {
        self.step_size_id
    }

    pub fn closest_sample(&self, pos: Vec3) -> Option<Sample> {
        self.reference_samples
            .iter()
            .fold_first(|closest_so_far, next_sample| {
                closer_sample(closest_so_far, next_sample, pos)
            })
            .cloned()
    }

    pub fn draw_on(&self, canvas: &Canvas, reference_color: Color32) {
        self.reference_samples
            .iter()
            .for_each(|sample| canvas.dot(sample.s, reference_color));
    }
}

fn closer_sample<'t>(s1: &'t Sample, s2: &'t Sample, pos: Vec3) -> &'t Sample {
    if (s1.s - pos).length() < (s2.s - pos).length() {
        s1
    } else {
        s2
    }
}
