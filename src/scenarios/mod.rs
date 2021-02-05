use crate::{Acceleration, Canvas, IntegrationParameters, Sample};
use bevy::{ecs::Entity, math::Vec3};
use egui::{Color32, Stroke};
use std::ops::Deref;

pub mod center_mass;

pub struct Scenario {
    accel: Box<dyn Acceleration>,
    start_pos: Vec3,
    start_velocity: Vec3,
    last_params: IntegrationParameters,
    trajectory: Vec<Vec3>,
    exact_step_samples: Vec<Sample>,
}

pub struct ScenarioId(pub Entity);

impl Scenario {
    const STEPS_PER_DT: usize = 100;

    pub fn new(
        acceleration: Box<dyn Acceleration>,
        start_position: Vec3,
        start_velocity: Vec3,
    ) -> Self {
        Self {
            accel: acceleration,
            start_pos: start_position,
            start_velocity,
            last_params: IntegrationParameters {
                num_steps: 0,
                step_duration: 0.,
            },
            trajectory: Vec::new(),
            exact_step_samples: Vec::new(),
        }
    }

    pub fn acceleration(&self) -> &dyn Acceleration {
        self.accel.deref()
    }

    pub fn sample_bounding_box(&mut self, params: &IntegrationParameters) -> BoundingBox {
        self.calculate_trajectory(params);
        let mut bbox = BoundingBox::default();
        self.trajectory.iter().for_each(|&s| bbox.expand_to(s));
        bbox
    }

    pub fn draw_on(
        &mut self,
        canvas: &mut Canvas,
        params: &IntegrationParameters,
        stroke: Stroke,
        sample_color: Color32,
    ) {
        self.calculate_trajectory(params);

        // fold_first is unstable. might be renamed to "reduce"
        // https://github.com/rust-lang/rust/pull/79805
        self.trajectory.iter().fold_first(|sample0, sample1| {
            canvas.line_segment(*sample0, *sample1, stroke);
            sample1
        });
        self.exact_step_samples
            .iter()
            .for_each(|sample| canvas.dot(sample.s, sample_color));
    }

    pub fn closest_sample(&self, pos: Vec3) -> Option<Sample> {
        self.exact_step_samples
            .iter()
            .fold_first(|closest_so_far, next_sample| {
                closer_sample(closest_so_far, next_sample, pos)
            })
            .cloned()
    }

    fn calculate_trajectory(&mut self, params: &IntegrationParameters) {
        if self.last_params == *params {
            return;
        }
        self.last_params = *params;
        self.trajectory = Vec::with_capacity(Self::STEPS_PER_DT * params.num_steps);
        self.exact_step_samples = Vec::with_capacity(params.num_steps + 1);

        let mut t0 = 0f32;
        let mut s0 = self.start_pos;
        let mut v0 = self.start_velocity;
        let a0 = self.accel.value_at(s0);
        self.trajectory.push(s0);
        self.exact_step_samples
            .push((0, t0, params.step_duration, s0, v0, a0).into());

        for step in 1..=params.num_steps {
            let t1 = step as f32 * params.step_duration;
            let mut ti0 = t0;
            for istep in 1..=Self::STEPS_PER_DT {
                let ti1 = t0 * ((Self::STEPS_PER_DT - istep) as f32 / Self::STEPS_PER_DT as f32)
                    + t1 * (istep as f32 / Self::STEPS_PER_DT as f32);
                let h = ti1 - ti0;

                let a0 = self.accel.value_at(s0);
                // let v1_tmp = v0 + a0 * h;
                // let s1_tmp = s0 + v0 * h + a0 * h * h; // std. Euler.  Good for circles
                let s1_tmp = s0 + v0 * h + 0.5 * a0 * h * h; // Exact for uniform acceleration
                let a1 = self.accel.value_at(s1_tmp);
                let v1 = v0 + 0.5 * (a0 + a1) * h;
                let s1 = s0 + v0 * h + (2. * a0 + a1) / 6. * h * h;

                s0 = s1;
                v0 = v1;
                ti0 = ti1;
                self.trajectory.push(s0);
            }
            t0 = t1;
            self.exact_step_samples
                .push((step, ti0, params.step_duration, s0, v0, a0).into());
        }
    }
}

fn closer_sample<'t>(s1: &'t Sample, s2: &'t Sample, pos: Vec3) -> &'t Sample {
    if (s1.s - pos).length() < (s2.s - pos).length() {
        s1
    } else {
        s2
    }
}

#[derive(Debug, Default)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn expand_to(&mut self, s: Vec3) {
        self.min.x = self.min.x.min(s.x);
        self.min.y = self.min.y.min(s.y);
        self.min.z = self.min.z.min(s.z);
        self.max.x = self.max.x.max(s.x);
        self.max.y = self.max.y.max(s.y);
        self.max.z = self.max.z.max(s.z);
    }

    pub fn center(&self) -> Vec3 {
        0.5 * (self.max + self.min)
    }

    pub fn diameter(&self) -> f32 {
        (self.max.x - self.min.x)
            .max(self.max.y - self.min.y)
            .max(self.max.z - self.min.z)
    }
}
