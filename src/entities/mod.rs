use bevy_ecs::World;

pub mod canvas;
pub mod integration;
pub mod integrator;
pub mod scenario;
pub mod step_size;

pub mod prelude {
    pub use super::canvas;
    pub use super::integration;
    pub use super::integrator;
    pub use super::scenario;
    pub use super::step_size;
    pub use super::Gather;
}

pub trait Gather<'a> {
    type T;
    fn gather_from(&self, world: &'a World) -> Self::T;
}
