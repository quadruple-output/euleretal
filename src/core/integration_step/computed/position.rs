use super::{
    contributions,
    core::{self, Fraction},
    Step,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    data: &'a Position,
}

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Position {
    pub(in crate::core::integration_step) s: core::Position,
    pub(in crate::core::integration_step) dt_fraction: Fraction,
    pub(in crate::core::integration_step) contributions: contributions::position::Collection,
}

impl<'a> Abstraction<'a> {
    pub fn s(&self) -> core::Position {
        self.data.s
    }

    pub fn dt_fraction(&self) -> Fraction {
        self.data.dt_fraction
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = contributions::position::Abstraction> {
        self.data
            .contributions
            .iter()
            .map(move |contribution| contribution.abstraction_for(self.step))
    }
}

impl<'a> PartialEq for Abstraction<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.data, other.data)
    }
}

impl Position {
    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction { step, data: self }
    }
}
