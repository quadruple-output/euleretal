use super::{
    core::{self, Position},
    quantity_contributions,
    step::{PositionRef, Step},
};

pub struct Velocity<'a> {
    step: &'a Step,
    data: &'a Data,
}

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Data {
    pub(in crate::core::integration_step) v: core::Velocity,
    pub(in crate::core::integration_step) sampling_position: PositionRef,
    pub(in crate::core::integration_step) contributions:
        Vec<quantity_contributions::velocity::Variant>,
}

impl<'a> Velocity<'a> {
    pub fn v(&self) -> core::Velocity {
        self.data.v
    }

    pub fn sampling_position(&self) -> Position {
        self.step[self.data.sampling_position].s
    }

    pub fn contributions_iter(
        &'a self,
    ) -> impl Iterator<Item = quantity_contributions::velocity::Abstraction<'a>> {
        self.data
            .contributions
            .iter()
            .map(move |contrib| contrib.abstraction_for(self.step))
    }
}

impl ::std::ops::Mul<&Data> for f32 {
    type Output = core::Velocity;

    fn mul(self, rhs: &Data) -> Self::Output {
        self * rhs.v
    }
}

impl Data {
    pub(in crate::core::integration_step) fn public_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Velocity<'a> {
        Velocity { step, data: self }
    }
}
