use super::{
    core::{self, Fraction},
    PositionContribution, PositionContributionData, Step,
};

pub struct Position<'a> {
    step: &'a Step,
    data: &'a Data,
}

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Data {
    pub(in crate::core::integration_step) s: core::Position,
    pub(in crate::core::integration_step) dt_fraction: Fraction,
    pub(in crate::core::integration_step) contributions: Vec<PositionContributionData>,
}

impl<'a> Position<'a> {
    pub fn s(&self) -> core::Position {
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

impl<'a> PartialEq for Position<'a> {
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self.data, other.data)
    }
}

impl Data {
    pub(in crate::core::integration_step) fn public_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Position<'a> {
        Position { step, data: self }
    }
}
