use super::{
    samples::{NewSampleWithPoints, StartCondition},
    AccelerationField,
};
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait Integrator: Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate_step(
        &self,
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    );

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
}
