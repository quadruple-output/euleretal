pub mod acceleration_field;
pub mod integrator;
pub mod samples;

pub mod prelude {
    pub use super::acceleration_field::AccelerationField;
    pub use super::integrator::Integrator;
    pub use super::samples::{CompleteSample, Samples};
    pub use super::{Acceleration, Position, Velocity};
}

pub type Position = crate::Vec3;
pub type Acceleration = crate::Vec3;
pub type Velocity = crate::Vec3;
