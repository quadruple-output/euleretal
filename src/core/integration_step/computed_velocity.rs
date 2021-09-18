use super::{
    core::{Position, Velocity},
    IntegrationStep, PositionRef, VelocityContribution, VelocityContributionData,
};

pub struct ComputedVelocity<'a> {
    step: &'a IntegrationStep,
    data: &'a Data,
}

pub struct Data {
    pub(super) v: Velocity,
    pub(super) sampling_position: PositionRef,
    pub(super) contributions: Vec<VelocityContributionData>,
}

impl<'a> ComputedVelocity<'a> {
    pub fn v(&self) -> Velocity {
        self.data.v
    }

    pub fn sampling_position(&self) -> Position {
        self.step[self.data.sampling_position].s
    }

    pub fn contributions_iter(&'a self) -> impl Iterator<Item = VelocityContribution<'a>> {
        self.data
            .contributions
            .iter()
            .map(move |contrib| contrib.public_for(self.step))
    }
}

impl ::std::ops::Mul<&Data> for f32 {
    type Output = Velocity;

    fn mul(self, rhs: &Data) -> Self::Output {
        self * rhs.v
    }
}

impl Data {
    pub(super) fn public_for<'a>(&'a self, step: &'a IntegrationStep) -> ComputedVelocity<'a> {
        ComputedVelocity { step, data: self }
    }
}
