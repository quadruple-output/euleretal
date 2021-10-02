use super::{
    super::integration_step::{
        step::{AccelerationRef, PositionRef, VelocityRef},
        PositionContributionData, PositionContributionDataCollection, Step,
    },
    core::{self, Fraction},
};

pub struct Position<'a> {
    step: &'a mut Step,
    dt_fraction: Fraction,
    contributions: PositionContributionDataCollection,
}

impl<'a> Position<'a> {
    pub fn new(step: &'a mut Step, dt_fraction: Fraction) -> Self {
        Self {
            step,
            dt_fraction,
            // most of the times there will be 3 contributions:
            contributions: PositionContributionDataCollection::with_capacity(3),
        }
    }

    pub fn based_on(mut self, s_ref: PositionRef) -> Self {
        self.contributions
            .push(PositionContributionData::StartPosition { s_ref });
        self
    }

    pub fn add_velocity_dt(mut self, v_ref: VelocityRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionData::VelocityDt {
                factor,
                v_ref,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn add_acceleration_dt_dt(mut self, a_ref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionData::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> PositionRef {
        let mut s = core::Position::origin();
        for contrib in self.contributions.iter() {
            s += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_position(s, self.dt_fraction, self.contributions)
    }
}

pub struct PositionContribution {
    inner: PositionContributionData,
}

impl From<PositionContributionData> for PositionContribution {
    fn from(data: PositionContributionData) -> Self {
        Self { inner: data }
    }
}

impl From<PositionRef> for PositionContribution {
    fn from(s_ref: PositionRef) -> Self {
        PositionContributionData::StartPosition { s_ref }.into()
    }
}

impl From<PositionContribution> for PositionContributionData {
    fn from(p: PositionContribution) -> Self {
        p.inner
    }
}

impl std::ops::Add<PositionContribution> for PositionContribution {
    type Output = PositionContributionDataCollection;

    fn add(self, rhs: PositionContribution) -> Self::Output {
        PositionContributionDataCollection(vec![self.inner, rhs.inner])
    }
}
