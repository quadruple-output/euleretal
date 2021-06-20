use super::{samples::StartCondition, AccelerationField, Duration, IntegrationStep};
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait Integrator: Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        acceleration_field: &dyn AccelerationField,
    ) -> IntegrationStep;

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }

    /// Number of acceleration values involved in computing the next sample. This does not include
    /// the acceleration value at the computed next sample.
    fn expected_accelerations_for_step(&self) -> usize;

    /// Number of positions involved in computing the next sample. This doen not include the
    /// position of the next sample.
    fn expected_positions_for_step(&self) -> usize;

    /// Number of velocity values involved in computing the next sample. This does not include the
    /// computed velocity of the next sample.
    fn expected_velocities_for_step(&self) -> usize;

    fn expected_capacities_for_step(&self) -> ExpectedCapacities {
        ExpectedCapacities {
            positions: self.expected_positions_for_step(),
            velocities: self.expected_velocities_for_step(),
            accelerations: self.expected_accelerations_for_step(),
        }
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
