use crate::acceleration::Acceleration;

pub mod center_mass;

pub struct Scenario {
    pub acceleration: Box<dyn Acceleration>,
}
