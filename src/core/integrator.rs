use super::integration_step::builders;
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait Integrator: Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate_step(
        &self,
        s0: builders::Position,
        v0: builders::Velocity,
        a0: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        builder: &mut builders::Step,
    );

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }
}

#[derive(Clone, Copy)]
pub struct ExpectedCapacities {
    pub positions: usize,
    pub velocities: usize,
    pub accelerations: usize,
}

impl Default for ExpectedCapacities {
    fn default() -> Self {
        Self {
            positions: 1,
            velocities: 1,
            accelerations: 1,
        }
    }
}
