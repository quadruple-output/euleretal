use crate::{
    core::samples::{NewSample, WithoutCalibrationPoints},
    prelude::*,
};

pub struct Kind;
pub mod comp {
    pub type Acceleration = Box<dyn super::AccelerationField>;
    pub type StartPosition = super::StartPosition;
    pub type StartVelocity = super::StartVelocity;
    pub type Duration = super::Duration;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

#[derive(bevy_ecs::Bundle)]
pub struct Bundle(
    pub Kind,
    pub comp::Acceleration,
    pub comp::StartPosition,
    pub comp::StartVelocity,
    pub comp::Duration,
);

pub type Query<'a> = (
    bevy_ecs::Entity,
    &'a Kind,
    &'a comp::Acceleration,
    &'a comp::StartPosition,
    &'a comp::StartVelocity,
    &'a comp::Duration,
);

pub struct Gathered<'a> {
    pub id: bevy_ecs::Entity,
    pub acceleration: &'a dyn AccelerationField,
    pub start_position: ChangeTracker<Vec3, change_tracker::Read>,
    pub start_velocity: ChangeTracker<Vec3, change_tracker::Read>,
    pub duration: ChangeTracker<R32, change_tracker::Read>,
}

impl Bundle {
    pub fn spawn(self, world: &mut bevy_ecs::World) -> self::Entity {
        Entity(world.spawn(self))
    }
}

impl<'a> super::Gather<'a> for Entity {
    type T = Gathered<'a>;

    fn gather_from(&self, world: &'a World) -> Gathered<'a> {
        // enforce type check for assignments:
        let acceleration = world.get::<comp::Acceleration>(self.0).unwrap();
        let start_position = world.get::<comp::StartPosition>(self.0).unwrap();
        let start_velocity = world.get::<comp::StartVelocity>(self.0).unwrap();
        let duration = world.get::<comp::Duration>(self.0).unwrap();
        Gathered {
            id: self.0,
            acceleration: &**acceleration,
            start_position: start_position.0.copy_read_only(),
            start_velocity: start_velocity.0.copy_read_only(),
            duration: duration.0.copy_read_only(),
        }
    }
}

impl<'a> super::Gather<'a> for Query<'a> {
    type T = Gathered<'a>;

    fn gather_from(&self, _: &'a World) -> Gathered<'a> {
        // enforce type check for assignments:
        let id: bevy_ecs::Entity = self.0;
        let acceleration: &'a comp::Acceleration = self.2;
        let start_position: &comp::StartPosition = self.3;
        let start_velocity: &comp::StartVelocity = self.4;
        let duration: &comp::Duration = self.5;
        Gathered {
            id,
            acceleration: &**acceleration,
            start_position: start_position.0.copy_read_only(),
            start_velocity: start_velocity.0.copy_read_only(),
            duration: duration.0.copy_read_only(),
        }
    }
}

impl<'a> Gathered<'a> {
    pub fn label(&self) -> String {
        self.acceleration.label()
    }
}

const STEPS_PER_DT: usize = 40;

pub fn calculate_trajectory(
    acceleration: &dyn AccelerationField,
    start_position: &ChangeTracker<Vec3, impl change_tracker::TRead>,
    start_velocity: &ChangeTracker<Vec3, impl change_tracker::TRead>,
    duration: &ChangeTracker<R32, impl change_tracker::TRead>,
    min_dt: R32,
) -> Vec<Vec3> {
    #[allow(clippy::cast_sign_loss)]
    let num_steps =
        (duration.get() / min_dt * R32::from(STEPS_PER_DT as f32)).into_inner() as usize;
    let (trajectory, _samples) = calculate_trajectory_and_samples(
        acceleration,
        start_position.get(),
        start_velocity.get(),
        1,
        duration.get(),
        num_steps,
    );
    log::info!("Calculated trajectory with {} segments", trajectory.len(),);
    trajectory
}

pub fn calculate_reference_samples(
    acceleration: &dyn AccelerationField,
    start_position: Vec3,
    start_velocity: Vec3,
    duration: R32,
    dt: R32,
) -> Samples {
    #[allow(clippy::cast_sign_loss)]
    let num_iterations = (duration / dt).into_inner() as usize;
    let (trajectory, samples) = calculate_trajectory_and_samples(
        acceleration,
        start_position,
        start_velocity,
        num_iterations,
        dt,
        STEPS_PER_DT,
    );
    log::info!(
        "Calculated {} reference samples, using trajectory with {} segments",
        samples.step_points().len(),
        trajectory.len(),
    );
    samples
}

/// returns (trajectory, samples)
fn calculate_trajectory_and_samples(
    acceleration: &dyn AccelerationField,
    start_position: Vec3,
    start_velocity: Vec3,
    iterations: usize,
    dt: R32,
    steps_per_dt: usize,
) -> (Vec<Vec3>, Samples) {
    let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
    let mut samples = Samples::<WithoutCalibrationPoints>::with_capacity(iterations);

    let mut t0 = R32::from(0.);
    let mut s0 = start_position;
    let mut v0 = start_velocity;
    let mut a0 = acceleration.value_at(s0);
    trajectory.push(s0);
    samples.push_sample(&NewSample {
        time: t0,
        dt,
        position: s0,
        velocity: v0,
        acceleration: a0,
    });

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
        samples.push_sample(&NewSample {
            time: t0,
            dt,
            position: s0,
            velocity: v0,
            acceleration: a0,
        });
    }
    (trajectory, samples.finalize())
}
