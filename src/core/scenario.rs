use super::{
    integrator::ExpectedCapacities, AccelerationField, Duration, Position, Samples, StartCondition,
    Step, Velocity,
};
use ::std::{collections::hash_map::DefaultHasher, hash::Hash};

pub struct Scenario {
    pub acceleration: Box<dyn AccelerationField>,
    pub start_position: Position,
    pub start_velocity: Velocity,
    pub duration: Duration,
}

const STEPS_PER_DT: usize = 40;

impl Scenario {
    pub fn label(&self) -> String {
        self.acceleration.label()
    }

    pub fn hash_default(&self, state: &mut DefaultHasher) {
        self.acceleration.hash(state);
        self.start_position.hash(state);
        self.start_velocity.hash(state);
        self.duration.hash(state);
    }

    pub fn calculate_trajectory(&self, min_dt: Duration) -> Vec<Position> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (self.duration / min_dt * STEPS_PER_DT as f32) as usize;
        let (trajectory, _samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            self.start_position,
            self.start_velocity,
            1,
            self.duration,
            num_steps,
        );
        log::info!("Calculated trajectory with {} segments", trajectory.len(),);
        trajectory
    }

    pub fn calc_intermediate_sample(
        &self,
        start_condition: &StartCondition,
        dt: Duration,
    ) -> (Position, Velocity) {
        let (_, samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            start_condition.position(),
            start_condition.velocity(),
            1,
            dt,
            STEPS_PER_DT,
        );
        let sample = samples.at(0);
        (
            sample.last_computed_position().s(),
            sample.last_computed_velocity().v(),
        )
    }

    pub fn calculate_reference_samples(&self, dt: Duration) -> Samples {
        #[allow(clippy::cast_sign_loss)]
        let num_iterations = (self.duration / dt) as usize;
        let (trajectory, samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            self.start_position,
            self.start_velocity,
            num_iterations,
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
}

/// returns (trajectory, samples)
fn calculate_trajectory_and_samples(
    acceleration: &dyn AccelerationField,
    start_position: Position,
    start_velocity: Velocity,
    iterations: usize,
    dt: Duration,
    steps_per_dt: usize,
) -> (Vec<Position>, Samples) {
    let mut t0 = 0.0.into();
    let mut s0 = start_position;
    let mut v0 = start_velocity;
    let mut a0 = acceleration.value_at(s0);

    let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
    trajectory.push(s0);
    let mut samples = Samples::new(iterations);

    let step_capacities = ExpectedCapacities {
        positions: 2,
        velocities: 2,
        accelerations: 2,
    };
    for step_count in 1..=iterations {
        let t1 = (step_count as f32) * dt;
        let mut new_step = Step::new(step_capacities, dt);
        new_step.initial_condition(&StartCondition::new(s0, v0, a0));
        let mut ti0 = t0;
        for intermediate_step_count in 1..=steps_per_dt {
            let ti1 = t0 * ((steps_per_dt - intermediate_step_count) as f32 / steps_per_dt as f32)
                + t1 * (intermediate_step_count as f32 / steps_per_dt as f32);
            let h = ti1 - ti0;

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
        new_step.raw_end_condition(s0, v0, a0);
        samples.push_sample(new_step);
    }

    (trajectory, samples.finalized())
}
