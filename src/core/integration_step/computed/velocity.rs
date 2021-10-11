use super::{
    contributions,
    core::{self, DtFraction, Position},
    step::{PositionRef, Step},
};

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Velocity {
    pub(in crate::core::integration_step) v: core::Velocity,
    pub(in crate::core::integration_step) sampling_position: PositionRef,
    contributions: contributions::velocity::Collection<1, 1>,
    contributions_scale: f32,
}

impl Velocity {
    pub(in super::super) fn new<const N: usize, const D: usize>(
        v: core::Velocity,
        sampling_position: PositionRef,
        dt_fraction: DtFraction<N, D>,
        contributions: contributions::velocity::Collection<N, D>,
    ) -> Self {
        #[allow(clippy::cast_precision_loss)]
        Self {
            v,
            sampling_position,
            contributions: contributions.transmute(),
            contributions_scale: dt_fraction.into(),
        }
    }

    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            velocity: self,
        }
    }

    pub(in crate::core::integration_step) fn has_contributions(&self) -> bool {
        !self.contributions.is_empty()
    }
}

pub struct Abstraction<'a> {
    step: &'a Step,
    velocity: &'a Velocity,
}

impl<'a> Abstraction<'a> {
    pub fn v(&self) -> core::Velocity {
        self.velocity.v
    }

    pub fn sampling_position(&self) -> Position {
        self.step[self.velocity.sampling_position].s
    }

    pub fn contributions_iter(
        &'a self,
    ) -> impl Iterator<Item = contributions::velocity::Abstraction<'a>> {
        self.velocity.contributions.iter().map(move |contrib| {
            contrib.abstraction_scaled_for(self.step, self.velocity.contributions_scale)
        })
    }
}
