use super::core::{self, Acceleration, Duration, Position, StartCondition, Velocity};

pub struct Step {
    dt: Duration,
    start_condition: StartCondition,
}

impl Step {
    pub fn new(start_condition: StartCondition, dt: Duration) -> Self {
        Self {
            dt,
            start_condition,
        }
    }

    pub fn result(self) -> core::Step {
        todo!()
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
