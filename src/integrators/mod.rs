use crate::{Acceleration, Sample};
pub use euler::*;

mod euler;

pub trait Integrator: Send + Sync {
    fn integrate<A: Acceleration>(a: A, s0: Sample, dt: f32) -> Sample;
}
