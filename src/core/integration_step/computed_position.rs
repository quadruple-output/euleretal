use super::{
    core::{Fraction, Position},
    PositionContribution, PositionContributionData, IntegrationStep,
};

pub struct ComputedPosition<'a> {
    step: &'a IntegrationStep,
    data: &'a Data,
}

impl<'a> ComputedPosition<'a> {
    pub fn s(&self) -> Position {
        self.data.s
    }

    pub fn dt_fraction(&self) -> Fraction {
        self.data.dt_fraction
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = PositionContribution> {
        self.data
            .contributions
            .iter()
            .map(move |contrib| contrib.public_for(self.step))
    }
}

impl<'a> PartialEq for ComputedPosition<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.data, other.data)
    }
}

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Data {
    pub(super) s: Position,
    pub(super) dt_fraction: Fraction,
    pub(super) contributions: Vec<PositionContributionData>,
}

impl Data {
    pub(super) fn public_for<'a>(&'a self, step: &'a IntegrationStep) -> ComputedPosition<'a> {
        ComputedPosition { step, data: self }
    }
}
