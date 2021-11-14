use crate::{Acceleration, Position, Velocity};

#[derive(Clone, Debug, PartialEq)]
pub struct StartCondition {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl StartCondition {
    #[must_use]
    pub fn new(position: Position, velocity: Velocity, acceleration: Acceleration) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }

    #[must_use]
    pub fn position(&self) -> Position {
        self.position
    }

    #[must_use]
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    #[must_use]
    pub fn acceleration(&self) -> Acceleration {
        self.acceleration
    }
}
