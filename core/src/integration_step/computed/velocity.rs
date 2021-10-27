use super::{
    contributions::{self, Contribution},
    step::{PositionRef, Step},
};
use crate::Position;

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Velocity {
    pub(in crate::integration_step) v: crate::Velocity,
    pub(in crate::integration_step) sampling_position: PositionRef,
    contributions: contributions::velocity::collection::Generic,
}

impl Velocity {
    pub(in crate::integration_step) fn new<const N: usize, const D: usize>(
        v: crate::Velocity,
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

    pub(in crate::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            velocity: self,
        }
    }

    pub(in crate::integration_step) fn has_contributions(&self) -> bool {
        !self.contributions.is_empty()
    }
}

pub struct Abstraction<'a> {
    step: &'a Step,
    velocity: &'a Velocity,
}

impl<'a> Abstraction<'a> {
    #[must_use]
    pub fn v(&self) -> crate::Velocity {
        self.velocity.v
    }

    #[must_use]
    pub fn sampling_position(&self) -> Position {
        self.step[self.velocity.sampling_position].s
    }

    /// note that the return value may live longer than self
    #[must_use]
    pub fn contributions_iter<'slf>(
        &'slf self,
    ) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a> {
        self.velocity.contributions.abstraction_iter_for(self.step)
    }
}
