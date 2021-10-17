use super::{
    contributions,
    core::{self, Fraction},
    Step,
};

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Position {
    pub(in super::super) s: core::Position,
    contributions: contributions::position::collection::Generic,
}

impl Position {
    pub(in super::super) fn new<const N: usize, const D: usize>(
        s: core::Position,
        contributions: contributions::position::Collection<N, D>,
    ) -> Self {
        Self {
            s,
            contributions: contributions.generalize(),
        }
    }

    pub(in super::super) fn abstraction_for<'a>(&'a self, step: &'a Step) -> Abstraction<'a> {
        Abstraction {
            step,
            position: self,
        }
    }

    pub(in super::super) fn has_contributions(&self) -> bool {
        !self.contributions.is_empty()
    }

    pub(in super::super) fn dt_fraction(&self) -> Fraction {
        self.contributions.dt_fraction()
    }
}

pub struct Abstraction<'a> {
    step: &'a Step,
    position: &'a Position,
}

impl<'a> Abstraction<'a> {
    pub fn s(&self) -> core::Position {
        self.position.s
    }

    pub fn dt_fraction(&self) -> Fraction {
        self.position.dt_fraction()
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = contributions::position::Abstraction> {
        self.position.contributions.abstraction_iter_for(self.step)
    }
}

impl<'a> PartialEq for Abstraction<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.position, other.position)
    }
}
