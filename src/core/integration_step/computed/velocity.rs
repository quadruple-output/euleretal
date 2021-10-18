use super::{
    contributions,
    core::{self, Position},
    step::{PositionRef, Step},
};

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Velocity {
    pub(in super::super) v: core::Velocity,
    pub(in super::super) sampling_position: PositionRef,
    contributions: contributions::velocity::collection::Generic,
}

impl Velocity {
    pub(in super::super) fn new<const N: usize, const D: usize>(
        v: core::Velocity,
        sampling_position: PositionRef,
        contributions: contributions::velocity::Collection<N, D>,
    ) -> Self {
        #[allow(clippy::cast_precision_loss)]
        Self {
            v,
            sampling_position,
            contributions: contributions.generalize(),
        }
    }

    pub(in super::super) fn abstraction_for<'a>(&'a self, step: &'a Step) -> Abstraction<'a> {
        Abstraction {
            step,
            velocity: self,
        }
    }

    pub(in super::super) fn has_contributions(&self) -> bool {
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
        self.velocity.contributions.abstraction_iter_for(self.step)
    }
}
