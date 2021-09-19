use crate::core::integrator::ExpectedCapacities;

use super::core::{self, Acceleration, Duration, Position, StartCondition, Velocity};

pub struct Step {
    dt: Duration,
    start_condition: StartCondition,
    step: core::Step,
}

impl Step {
    pub fn new(start_condition: &StartCondition, dt: Duration) -> Self {
        let mut step = core::Step::new(ExpectedCapacities::default(), dt);
        let start_pos_ref = step.start_position(start_condition.position());
        step.start_velocity(start_condition.velocity(), start_pos_ref);
        step.start_acceleration(start_condition.acceleration(), start_pos_ref);
        Self {
            dt,
            start_condition: start_condition.clone(),
            step,
        }
    }

    pub fn result(self) -> core::Step {
        self.step
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn start_values(&self) -> (Position, Velocity, Acceleration) {
        (
            self.start_condition.position(),
            self.start_condition.velocity(),
            self.start_condition.acceleration(),
        )
    }
}
