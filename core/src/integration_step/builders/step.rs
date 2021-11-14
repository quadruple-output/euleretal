use super::{
    integration_step::{
        contributions,
        step::{AccelerationRef, PositionRef, VelocityRef},
    },
    DtFraction,
};
use crate::AccelerationField;

pub struct Step<'a> {
    acceleration_field: &'a dyn AccelerationField,
    step: &'a mut crate::Step,
}

impl<'a> Step<'a> {
    pub fn new(acceleration_field: &'a dyn AccelerationField, step: &'a mut crate::Step) -> Self {
        Self {
            acceleration_field,
            step,
        }
    }

    /// consumes `self`, and therefore cannot be called twice on the same instance.
    pub fn finalize(mut self) {
        self.set_display_position(self.step.last_velocity_ref(), self.step.last_position_ref());
        self.step
            .compute_acceleration_at_last_position(self.acceleration_field);
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub const fn dt(&self) -> DtFraction<1, 1> {
        DtFraction
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn start_values(&self) -> (PositionRef, VelocityRef, AccelerationRef) {
        (
            PositionRef::default(),
            VelocityRef::default(),
            AccelerationRef::default(),
        )
    }

    pub fn set_display_position(&mut self, v_ref: VelocityRef, s_ref: PositionRef) {
        self.step[v_ref].sampling_position = s_ref;
    }

    pub fn acceleration_at(&mut self, s_ref: PositionRef) -> AccelerationRef {
        self.step
            .add_computed_acceleration(self.acceleration_field.value_at(self.step[s_ref].s), s_ref)
    }
}

pub trait Collector<Contribution> {
    type Output;
    fn compute(&mut self, _: Contribution) -> Self::Output;
}

impl<'a, const N: usize, const D: usize> Collector<contributions::position::Collection<N, D>>
    for Step<'a>
{
    type Output = PositionRef;

    fn compute(
        &mut self,
        contributions: contributions::position::Collection<N, D>,
    ) -> Self::Output {
        let mut s = crate::Position::origin();
        for contrib in &contributions {
            s += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_position(s, DtFraction::<N, D>, contributions)
    }
}

impl<'a, const N: usize, const D: usize> Collector<contributions::velocity::Collection<N, D>>
    for Step<'a>
{
    type Output = VelocityRef;

    fn compute(
        &mut self,
        contributions: contributions::velocity::Collection<N, D>,
    ) -> Self::Output {
        let mut v = crate::Velocity::zeros();
        for contrib in &contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step.add_computed_velocity(
            v,
            self.step.last_position_ref(), // just a default. Can be overwritten.
            DtFraction::<N, D>,
            contributions,
        )
    }
}
