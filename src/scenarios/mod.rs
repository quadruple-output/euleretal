use std::ops::Deref;

use bevy::math::Vec3;
use egui::{Color32, Stroke};

use crate::{acceleration::Acceleration, canvas::Canvas, sample::Sample};

pub mod center_mass;

pub struct Scenario {
    accel: Box<dyn Acceleration>,
    start_pos: Vec3,
    start_velocity: Vec3,
    /// delta t (single time step duration)
    step_duration: f32,
    /// simulation duration (for drawing exact solution)
    num_steps: usize,
    trajectory: Vec<Sample>,
    exact_step_samples: Vec<Sample>,
}

impl Scenario {
    const STEPS_PER_DT: usize = 100;

    pub fn new(
        acceleration: Box<dyn Acceleration>,
        start_position: Vec3,
        start_velocity: Vec3,
        step_duration: f32,
        num_steps: usize,
    ) -> Self {
        let mut instance = Self {
            accel: acceleration,
            start_pos: start_position,
            start_velocity,
            step_duration,
            num_steps,
            trajectory: Vec::with_capacity(Self::STEPS_PER_DT * num_steps),
            exact_step_samples: Vec::with_capacity(num_steps + 1),
        };
        instance.calculate_trajectory();
        instance
    }

    pub fn acceleration(&self) -> &dyn Acceleration {
        self.accel.deref()
    }

    pub fn start_position(&self) -> Vec3 {
        self.start_pos
    }

    pub fn step_duration(&self) -> f32 {
        self.step_duration
    }

    pub fn draw_on(&self, canvas: &Canvas, stroke: Stroke, sample_color: Color32) {
        // fold_first is unstable. might be renamed to "reduce"
        // https://github.com/rust-lang/rust/pull/79805
        self.trajectory.iter().fold_first(|sample0, sample1| {
            canvas.line_segment(sample0.s, sample1.s, stroke);
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

    fn calculate_trajectory(&mut self) {
        self.trajectory.clear();
        self.exact_step_samples.clear();

        let mut t0 = 0f32;
        let mut s0 = self.start_pos;
        let mut v0 = self.start_velocity;
        self.trajectory.push((s0, v0, t0).into());
        self.exact_step_samples.push((s0, v0, t0).into());

        for step in 1..=self.num_steps {
            let t1 = step as f32 * self.step_duration;
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
                self.trajectory.push((s0, v0, ti0).into());
            }
            t0 = t1;
            self.exact_step_samples.push((s0, v0, t0).into());
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
