use crate::prelude::*;

pub struct Kind;
pub mod comp {
    pub type Acceleration = Box<dyn super::Acceleration>;
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

impl Bundle {
    pub fn spawn(self, commands: &mut bevy_ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}

const STEPS_PER_DT: usize = 40;

pub fn calculate_trajectory(
    acceleration: &dyn Acceleration,
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
    acceleration: &dyn Acceleration,
    start_position: Vec3,
    start_velocity: Vec3,
    duration: R32,
    dt: R32,
) -> Vec<Sample> {
    #[allow(clippy::cast_sign_loss)]
    let (trajectory, samples) = calculate_trajectory_and_samples(
        acceleration,
        start_position,
        start_velocity,
        (duration / dt).into_inner() as usize,
        dt,
        STEPS_PER_DT,
    );
    log::info!(
        "Calculated {} reference samples, using trajectory with {} segments",
        samples.len(),
        trajectory.len(),
    );
    samples
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
