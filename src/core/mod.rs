use super::import;

mod acceleration;
mod acceleration_field;
mod dt_fraction;
mod duration;
mod fraction;
mod integration;
pub mod integration_step;
mod integrator;
mod r#move;
mod obj;
mod position;
pub mod samples;
mod scenario;
mod vector_quantity;
mod velocity;

pub use acceleration::Acceleration;
pub use acceleration_field::AccelerationField;
pub use dt_fraction::DtFraction;
pub use duration::Duration;
pub use fraction::Fraction;
pub use integration::Integration;
pub use integration_step::{StartCondition, Step};
pub use integrator::Integrator;
pub use obj::Obj;
pub use position::Position;
pub use r#move::Move;
pub use samples::Samples;
pub use scenario::Scenario;
pub use velocity::Velocity;

pub enum PhysicalQuantityKind {
    Position,
    Velocity,
    Acceleration,
}

use super::core; // explicit self-use to make `core` (==self) available in submodules as `super::core`
use vector_quantity::VectorQuantity;
