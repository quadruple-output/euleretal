use super::import;

#[macro_use]
mod fraction; // mods with macros need to go first
mod acceleration_field;
pub mod derived_quantities;
mod duration;
mod integration;
mod integration_step;
mod integrator;
mod obj;
pub mod samples;
mod scenario;
mod start_position;
mod start_velocity;

pub use acceleration_field::AccelerationField;
pub use duration::Duration;
pub use fraction::Fraction;
pub use integration::Integration;
pub use integration_step::IntegrationStep;
pub use integrator::Integrator;
pub use obj::Obj;
pub use samples::{NewSampleWithPoints, Samples, StartCondition};
pub use scenario::Scenario;
pub use start_position::StartPosition;
pub use start_velocity::StartVelocity;

// todo: convert these types to structs:
pub type Position = import::Vec3;
pub type Acceleration = import::Vec3;
pub type Velocity = import::Vec3;
