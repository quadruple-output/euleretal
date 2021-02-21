use crate::{Acceleration, ChangeTracker, Sample, TrackedChange};
use bevy::math::Vec3;
use egui::{Slider, Ui};
use std::ops::Deref;

mod center_mass;
mod constant_acceleration;

pub use center_mass::CenterMass;
pub use constant_acceleration::ConstantAcceleration;

pub struct Scenario {
    accel: Box<dyn Acceleration>,
    start_position: ChangeTracker<Vec3>,
    start_velocity: ChangeTracker<Vec3>,
    duration: ChangeTracker<f32>,
}

impl TrackedChange for Scenario {
    fn change_count(&self) -> crate::change_tracker::ChangeCount {
        self.start_position.change_count()
            + self.start_velocity.change_count()
            + self.duration.change_count()
    }
}

impl Scenario {
    const STEPS_PER_DT: usize = 40;

    pub fn new(
        acceleration: Box<dyn Acceleration>,
        start_position: Vec3,
        start_velocity: Vec3,
        duration: f32,
    ) -> Self {
        Self {
            accel: acceleration,
            start_position: ChangeTracker::with(start_position),
            start_velocity: ChangeTracker::with(start_velocity),
            duration: ChangeTracker::with(duration),
        }
    }

    pub fn show_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(self.acceleration().label());
            ui.vertical(|ui| {
                let mut duration = self.duration.get();
                ui.add(
                    Slider::f32(&mut duration, 0.1..=50.)
                        .logarithmic(true)
                        .text("duration"),
                );
                self.duration.set(duration);
            });
        });
    }

    pub fn acceleration(&self) -> &dyn Acceleration {
        self.accel.deref()
    }

    pub fn s0(&self) -> Vec3 {
        self.start_position.get()
    }

    pub fn v0(&self) -> Vec3 {
        self.start_velocity.get()
    }

    pub fn duration(&self) -> f32 {
        self.duration.get()
    }

    pub fn calculate_trajectory(&self, min_dt: f32) -> Vec<Vec3> {
        let num_steps = (self.duration.get() / min_dt * Self::STEPS_PER_DT as f32) as usize;
        let (trajectory, _samples) = self._calculate_trajectory(1, self.duration.get(), num_steps);
        trajectory
    }

    pub fn calculate_reference_samples(&self, dt: f32) -> Vec<Sample> {
        let (_trajectory, samples) =
            self._calculate_trajectory((self.duration.get() / dt) as usize, dt, Self::STEPS_PER_DT);
        samples
    }

    /// returns (trajectory, samples)
    fn _calculate_trajectory(
        &self,
        iterations: usize,
        dt: f32,
        steps_per_dt: usize,
    ) -> (Vec<Vec3>, Vec<Sample>) {
        let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
        let mut samples = Vec::with_capacity(iterations + 1);

        let mut t0 = 0f32;
        let mut s0 = self.start_position.get();
        let mut v0 = self.start_velocity.get();
        let mut a0 = self.accel.value_at(s0);
        trajectory.push(s0);
        samples.push((0, t0, dt, s0, v0, a0).into());

        for step in 1..=iterations {
            let t1 = step as f32 * dt;
            let mut ti0 = t0;
            for istep in 1..=steps_per_dt {
                let ti1 = t0 * ((steps_per_dt - istep) as f32 / steps_per_dt as f32)
                    + t1 * (istep as f32 / steps_per_dt as f32);
                let h = ti1 - ti0;

                a0 = self.accel.value_at(s0);
                // let v1_tmp = v0 + a0 * h;
                // let s1_tmp = s0 + v0 * h + a0 * h * h; // std. Euler.  Good for circles
                let s1_tmp = s0 + v0 * h + 0.5 * a0 * h * h; // Exact for uniform acceleration
                let a1 = self.accel.value_at(s1_tmp);
                let v1 = v0 + 0.5 * (a0 + a1) * h;
                let s1 = s0 + v0 * h + (2. * a0 + a1) / 6. * h * h;

                ti0 = ti1;
                s0 = s1;
                v0 = v1;
                a0 = a1;
                trajectory.push(s0);
            }
            t0 = t1;
            samples.push((step, t0, dt, s0, v0, a0).into());
        }
        (trajectory, samples)
    }
}
