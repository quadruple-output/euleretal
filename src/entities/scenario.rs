use crate::{
    core::samples::{NewSample, StartCondition, WithoutCalibrationPoints},
    prelude::*,
};

pub struct Scenario {
    pub acceleration: Box<dyn AccelerationField>,
    pub start_position: StartPosition,
    pub start_velocity: StartVelocity,
    pub duration: Duration,
}

impl Scenario {
    pub fn label(&self) -> String {
        self.acceleration.label()
    }
}

const STEPS_PER_DT: usize = 40;

pub fn calculate_trajectory(
    acceleration: &dyn AccelerationField,
    // todo: don't use ChangeTracker but StartPosition. Impl. Deref for StartPosition
    start_position: &StartPosition,
    start_velocity: &StartVelocity,
    duration: &Duration,
    min_dt: R32,
) -> Vec<Vec3> {
    #[allow(clippy::cast_sign_loss)]
    let num_steps =
        (duration.get() / min_dt * R32::from(STEPS_PER_DT as f32)).into_inner() as usize;
    let (trajectory, _samples) = calculate_trajectory_and_samples(
        acceleration,
        start_position.0.get(),
        start_velocity.0.get(),
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
    let mut t0 = R32::from(0.);
    let mut s0 = start_position;
    let mut v0 = start_velocity;
    let mut a0 = acceleration.value_at(s0);

    let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
    trajectory.push(s0);
    let mut samples = Samples::<WithoutCalibrationPoints>::new(
        &StartCondition {
            position: s0,
            velocity: v0,
            acceleration: a0,
        },
        iterations,
    );

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
            dt,
            position: s0,
            velocity: v0,
            acceleration: a0,
        });
    }

    (trajectory, samples.finalized())
}
