use super::{contributions, Step};
use crate::Fraction;

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Position {
    pub(in crate::integration_step) s: crate::Position,
    contributions: contributions::position::collection::Generic,
}

impl Position {
    pub(in crate::integration_step) fn new<const N: usize, const D: usize>(
        s: crate::Position,
        contributions: contributions::position::Collection<N, D>,
    ) -> Self {
        Self {
            s,
            contributions: contributions.generalize(),
        }
    }

    pub(in crate::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            position: self,
        }
    }

    pub(in crate::integration_step) fn has_contributions(&self) -> bool {
        !self.contributions.is_empty()
    }

    pub(in crate::integration_step) fn dt_fraction(&self) -> Fraction {
        self.contributions.dt_fraction()
        // Todo: This is wrong. We have to distinguish between the "absolute"
        // dt_fraction (of the computed position) and the "relative"
        // dt_fraction (of the contributions)
    }
}

pub struct Abstraction<'a> {
    step: &'a Step,
    position: &'a Position,
}

impl<'a> Abstraction<'a> {
    #[must_use]
    pub fn s(&self) -> crate::Position {
        self.position.s
    }

    #[must_use]
    pub fn dt_fraction(&self) -> Fraction {
        self.position.dt_fraction()
    }

    /// note that the return value may live longer than self
    pub fn contributions_iter<'slf>(
        &'slf self,
    ) -> impl Iterator<Item = contributions::position::Abstraction<'a>> {
        self.position.contributions.abstraction_iter_for(self.step)
    }
}

impl<'a> PartialEq for Abstraction<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.position, other.position)
    }
}
