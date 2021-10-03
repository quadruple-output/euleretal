use super::{
    core::{self, Fraction},
    integration_step::{
        step::{AccelerationRef, PositionRef, Step, VelocityRef},
        PositionContributionData, VelocityContributionData,
    },
    position::PositionContribution,
    DtFraction,
};

pub struct Velocity<'a> {
    step: &'a mut Step,
    dt_fraction: Fraction,
    s_ref: PositionRef,
    contributions: Vec<VelocityContributionData>,
}

impl<'a> Velocity<'a> {
    pub fn new(step: &'a mut Step, dt_fraction: Fraction, s_ref: PositionRef) -> Self {
        Self {
            step,
            dt_fraction,
            s_ref,
            // most of the times there will be 2 contributions:
            contributions: Vec::with_capacity(2),
        }
    }

    pub fn based_on(mut self, v_ref: VelocityRef) -> Self {
        self.contributions
            .push(VelocityContributionData::Velocity { v_ref });
        self
    }

    pub fn add_acceleration_dt(mut self, a_ref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(VelocityContributionData::AccelerationDt {
                factor,
                a_ref,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> VelocityRef {
        let mut v = core::Velocity::zeros();
        for contrib in &self.contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_velocity(v, self.s_ref, self.contributions)
    }
}

pub struct VelocityContribution {
    inner: VelocityContributionData,
}

impl From<VelocityContributionData> for VelocityContribution {
    fn from(data: VelocityContributionData) -> Self {
        Self { inner: data }
    }
}

impl From<VelocityRef> for VelocityContribution {
    fn from(v_ref: VelocityRef) -> Self {
        VelocityContributionData::Velocity { v_ref }.into()
    }
}

impl From<VelocityContribution> for VelocityContributionData {
    fn from(v: VelocityContribution) -> Self {
        v.inner
    }
}

impl std::ops::Mul<DtFraction> for VelocityContribution {
    type Output = PositionContribution;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self.inner {
            VelocityContributionData::Velocity { v_ref } => PositionContributionData::VelocityDt {
                factor: 1., //todo
                v_ref,
                dt_fraction: dt_fraction.into(),
            }
            .into(),
            VelocityContributionData::AccelerationDt {
                factor,
                a_ref,
                dt_fraction: dt_fraction_lhs,
            } => {
                // todo: cannot handle `a * dt * dt_2` where dt != dt_2
                debug_assert_eq!(dt_fraction_lhs, dt_fraction.into());
                PositionContributionData::AccelerationDtDt {
                    factor,
                    a_ref,
                    dt_fraction: dt_fraction_lhs,
                }
                .into()
            }
        }
    }
}
