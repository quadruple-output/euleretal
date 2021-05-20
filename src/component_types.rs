pub mod duration;
pub mod start_position;
pub mod start_velocity;
pub mod step_size;

pub mod prelude {
    pub use super::duration::Duration;
    pub use super::start_position::StartPosition;
    pub use super::start_velocity::StartVelocity;
    pub use super::step_size::StepSize; // todo: does not belong here (→ ui::entities)
}
