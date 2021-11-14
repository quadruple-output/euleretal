#[cfg(feature = "persistence")]
use super::scenarios;
use super::{AccelerationField, Duration, Position, Samples, StartCondition, Step, Velocity};
use ::std::{collections::hash_map::DefaultHasher, hash::Hash};

#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct Scenario {
    #[cfg_attr(
        feature = "persistence",
        serde(with = "scenarios::serde_box_dyn_acceleration_field")
    )]
    pub acceleration: Box<dyn AccelerationField>,
    pub start_position: Position,
    pub start_velocity: Velocity,
    pub duration: Duration,
}

impl ::std::fmt::Debug for Scenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scenario")
            //.field("acceleration", &self.acceleration)
            .field("start_position", &self.start_position)
            .field("start_velocity", &self.start_velocity)
            .field("duration", &self.duration)
            .finish()
    }
}

const STEPS_PER_DT: usize = 40;

impl Scenario {
    #[must_use]
    pub fn label(&self) -> String {
        self.acceleration.label()
    }

    pub fn hash_default(&self, state: &mut DefaultHasher) {
        self.acceleration.hash(state);
        self.start_position.hash(state);
        self.start_velocity.hash(state);
        self.duration.hash(state);
    }

    #[must_use]
    pub fn calculate_trajectory(&self, min_dt: Duration) -> Vec<Position> {
        let start = ::std::time::Instant::now();
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let num_steps = (self.duration / min_dt * STEPS_PER_DT as f32) as usize;
        let (trajectory, _samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            self.start_position,
            self.start_velocity,
            1,
            self.duration,
            num_steps,
        );
        log::debug!(
            "{}: trajectory with {} segments: {}µs",
            self.label(),
            trajectory.len(),
            start.elapsed().as_micros()
        );
        trajectory
    }

    #[must_use]
    pub fn calc_intermediate_sample(&self, start_condition: &StartCondition, dt: Duration) -> Step {
        let (_, samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            start_condition.position(),
            start_condition.velocity(),
            1,
            dt,
            STEPS_PER_DT,
        );
        samples.at(0).clone()
    }

    #[must_use]
    pub fn calculate_reference_samples(&self, dt: Duration) -> Samples {
        let start = ::std::time::Instant::now();
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let num_iterations = (self.duration / dt) as usize;
        let (_trajectory, samples) = calculate_trajectory_and_samples(
            &*self.acceleration,
            self.start_position,
            self.start_velocity,
            num_iterations,
            dt,
            STEPS_PER_DT,
        );
        log::debug!(
            "{}: {} reference samples: {}µs",
            self.label(),
            samples.len(),
            start.elapsed().as_micros()
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
    #![allow(clippy::cast_precision_loss)]

    let mut t0 = 0.0.into();
    let mut s0 = start_position;
    let mut v0 = start_velocity;
    let mut a0 = acceleration.value_at(s0);

    let mut trajectory = Vec::with_capacity(iterations * steps_per_dt + 1);
    trajectory.push(s0);
    let mut samples = Samples::new(iterations);

    let mut step = Step::new(&StartCondition::new(s0, v0, a0), dt);
    let steps_per_dt_float = steps_per_dt as f32;
    let div_by_steps_per_dt = 1_f32 / steps_per_dt_float;
    let div_by_6 = 1_f32 / 6_f32;
    let mut t1 = Duration::default();
    for _ in 0..iterations {
        t1 += dt;
        // let mut new_step = Step::new_deprecated(step_capacities, dt);
        // new_step.set_start_condition(&StartCondition::new(s0, v0, a0));
        let mut ti0 = t0;
        let mut intermediate_step_count = 0_f32;
        for _ in 0..steps_per_dt {
            intermediate_step_count += 1_f32;
            let ti1 = (t0 * (steps_per_dt_float - intermediate_step_count)
                + t1 * intermediate_step_count)
                * div_by_steps_per_dt;
            let h = ti1 - ti0;

            a0 = acceleration.value_at(s0);
            // let v1_tmp = v0 + a0 * h;
            // let s1_tmp = s0 + v0 * h + a0 * h * h; // std. Euler.  Good for circles
            let s1_tmp = s0 + v0 * h + 0.5 * a0 * h * h; // Exact for uniform acceleration
            let a1 = acceleration.value_at(s1_tmp);
            let v1 = v0 + 0.5 * (a0 + a1) * h;
            let s1 = s0 + v0 * h + (2. * a0 + a1) * div_by_6 * h * h;

            ti0 = ti1;
            s0 = s1;
            v0 = v1;
            a0 = a1;
            trajectory.push(s0);
        }
        t0 = t1;
        step.raw_end_condition(s0, v0, a0);
        let next_step = step.create_next();
        samples.push_sample(step);
        step = next_step;
    }

    (trajectory, samples.finalized())
}
