use super::core::{Acceleration, Position, Velocity};

#[derive(Clone)]
pub struct StartCondition {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl StartCondition {
    pub fn new(position: Position, velocity: Velocity, acceleration: Acceleration) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn acceleration(&self) -> Acceleration {
        self.acceleration
    }
}
