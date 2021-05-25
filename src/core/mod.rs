pub mod acceleration_field;
pub mod integration;
pub mod integrator;
pub mod samples;
pub mod scenario;

pub use integration::Integration;
pub use integrator::Integrator;

pub mod prelude {
    pub use super::acceleration_field::AccelerationField;
    pub use super::samples::{CompleteSample, Samples};
    pub use super::scenario::{self, Scenario};
    pub use super::{Acceleration, Position, Velocity};
}

pub type Position = crate::Vec3;
pub type Acceleration = crate::Vec3;
pub type Velocity = crate::Vec3;
