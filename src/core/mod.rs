use super::import;

#[macro_use]
mod fraction; // mods with macros need to go first
mod acceleration_field;
mod duration;
mod integration;
mod integrator;
mod obj;
mod samples;
mod scenario;
mod start_position;
mod start_velocity;

pub use acceleration_field::AccelerationField;
pub use duration::Duration;
pub use fraction::Fraction;
pub use integration::Integration;
pub use integrator::{Integrator, OneStepDirect, OneStepWithCalibrationPoints};
pub use obj::Obj;
pub use samples::{
    CompleteSample, FinalizedCalibrationPoints, NewSample, NewSampleWithPoints, Samples,
    StartCondition,
};
pub use scenario::Scenario;
pub use start_position::StartPosition;
pub use start_velocity::StartVelocity;

// todo: convert these types to structs:
pub type Position = import::Vec3;
pub type Acceleration = import::Vec3;
pub type Velocity = import::Vec3;
