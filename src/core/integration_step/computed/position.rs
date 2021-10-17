use super::{
    contributions,
    core::{self, DtFraction, Fraction},
    Step,
};

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Position {
    pub(in super::super) s: core::Position,
    pub(in super::super) dt_fraction: Fraction,
    contributions: contributions::position::Collection<1, 1>, //todo: can this be a reference?
}

impl Position {
    pub(in super::super) fn new<const N: usize, const D: usize>(
        s: core::Position,
        dt_fraction: DtFraction<N, D>,
        contributions: contributions::position::Collection<N, D>,
    ) -> Self {
        let todo = &"bundle {dt_fraction, contributions} in new type CollectionDyn";
        Self {
            s,
            dt_fraction: dt_fraction.into(),
            contributions: contributions.transmute(),
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
        self.position.dt_fraction
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = contributions::position::Abstraction> {
        self.position.contributions.iter().map(move |contribution| {
            contribution.abstraction_scaled_for(self.step, self.position.dt_fraction.into())
        })
    }
}

impl<'a> PartialEq for Abstraction<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.position, other.position)
    }
}
