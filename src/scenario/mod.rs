use crate::{Acceleration, ChangeTracker, Duration, Sample, TrackedChange};
use bevy::math::Vec3;
use decorum::R32;
use log::info;

mod center_mass;
mod constant_acceleration;

pub use center_mass::CenterMass;
pub use constant_acceleration::ConstantAcceleration;

pub struct StartPosition(pub ChangeTracker<Vec3>);
pub struct StartVelocity(pub ChangeTracker<Vec3>);

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::ecs::Entity);

#[derive(bevy::ecs::Bundle)]
pub struct Bundle(
    pub Box<dyn Acceleration>,
    pub StartPosition,
    pub StartVelocity,
    pub Duration,
);

pub type Query<'a> = (
    &'a Box<dyn Acceleration>,
    &'a StartPosition,
    &'a StartVelocity,
    &'a Duration,
);

impl Bundle {
    pub fn spawn(self, commands: &mut bevy::ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}

impl<'a> TrackedChange for Query<'a> {
    fn change_count(&self) -> crate::change_tracker::ChangeCount {
        let (_, start_position, start_velocity, duration) = self;
        start_position.0.change_count()
            + start_velocity.0.change_count()
            + duration.0.change_count()
    }
}

pub trait Scenario {
    fn acceleration(&self) -> &dyn Acceleration;
    fn start_position(&self) -> &StartPosition;
    fn start_velocity(&self) -> &StartVelocity;
    fn duration(&self) -> &Duration;
    fn calculate_trajectory(&self, min_dt: R32) -> Vec<Vec3>;
    fn calculate_reference_samples(&self, dt: R32) -> Vec<Sample>;
}

const STEPS_PER_DT: usize = 40;

impl<'a> Scenario for Query<'a> {
    #[inline]
    fn acceleration(&self) -> &dyn Acceleration {
        &**self.0
    }

    #[inline]
    fn start_position(&self) -> &StartPosition {
        &self.1
    }

    #[inline]
    fn start_velocity(&self) -> &StartVelocity {
        &self.2
    }

    #[inline]
    fn duration(&self) -> &Duration {
        &self.3
    }

    fn calculate_trajectory(&self, min_dt: R32) -> Vec<Vec3> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (self.duration().0.get() / min_dt * R32::from(STEPS_PER_DT as f32))
            .into_inner() as usize;
        let (trajectory, _samples) = calculate_trajectory_and_samples(
            self.acceleration(),
            self.start_position().0.get(),
            self.start_velocity().0.get(),
            1,
            self.duration().0.get(),
            num_steps,
        );
        info!("Calculated trajectory with {} segments", trajectory.len(),);
        trajectory
    }

    fn calculate_reference_samples(&self, dt: R32) -> Vec<Sample> {
        #[allow(clippy::cast_sign_loss)]
        let (trajectory, samples) = calculate_trajectory_and_samples(
            self.acceleration(),
            self.start_position().0.get(),
            self.start_velocity().0.get(),
            (self.duration().0.get() / dt).into_inner() as usize,
            dt,
            STEPS_PER_DT,
        );
        info!(
            "Calculated {} reference samples, using trajectory with {} segments",
            samples.len(),
            trajectory.len(),
        );
        samples
    }
}

/// returns (trajectory, samples)
fn calculate_trajectory_and_samples(
    acceleration: &dyn Acceleration,
    start_position: Vec3,
    start_velocity: Vec3,
    iterations: usize,
    dt: R32,
    steps_per_dt: usize,
) -> (Vec<Vec3>, Vec<Sample>) {
    let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
    let mut samples = Vec::with_capacity(iterations + 1);

    let mut t0 = R32::from(0.);
    let mut s0 = start_position;
    let mut v0 = start_velocity;
    let mut a0 = acceleration.value_at(s0);
    trajectory.push(s0);
    samples.push((0, t0, dt, s0, v0, a0).into());

    for step in 1..=iterations {
        let t1 = R32::from(step as f32) * dt;
        let mut ti0 = t0;
        for istep in 1..=steps_per_dt {
            let ti1 = t0 * ((steps_per_dt - istep) as f32 / steps_per_dt as f32)
                + t1 * (istep as f32 / steps_per_dt as f32);
            let h = (ti1 - ti0).into_inner();

            a0 = acceleration.value_at(s0);
            // let v1_tmp = v0 + a0 * h;
            // let s1_tmp = s0 + v0 * h + a0 * h * h; // std. Euler.  Good for circles
            let s1_tmp = s0 + v0 * h + 0.5 * a0 * h * h; // Exact for uniform acceleration
            let a1 = acceleration.value_at(s1_tmp);
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
