use crate::{Acceleration, Sample};
use bevy::ecs::Entity;
pub use euler::*;

mod euler;

pub trait Integrator: Send + Sync {
    fn integrate<A: Acceleration>(a: A, s0: Sample, dt: f32) -> Sample;
}

pub struct IntegratorId(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegrationParameters {
    /// delta t (single time step duration)
    pub step_duration: f32,
    /// simulation duration (for drawing exact solution)
    pub num_steps: usize,
}

impl Default for IntegrationParameters {
    fn default() -> Self {
        Self {
            step_duration: 1.,
            num_steps: 10,
        }
    }
}
