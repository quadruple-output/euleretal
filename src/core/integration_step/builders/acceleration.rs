use super::{
    integration_step::{
        builders::velocity::VelocityContribution, step::AccelerationRef,
        AccelerationContributionData, VelocityContributionData,
    },
    DtFraction,
};

#[derive(Clone, Copy)]
pub struct AccelerationContribution {
    inner: AccelerationContributionData,
}

impl From<AccelerationContributionData> for AccelerationContribution {
    fn from(data: AccelerationContributionData) -> Self {
        Self { inner: data }
    }
}

impl From<AccelerationRef> for AccelerationContribution {
    fn from(a_ref: AccelerationRef) -> Self {
        AccelerationContributionData::Acceleration { a_ref }.into()
    }
}

impl From<AccelerationContribution> for AccelerationContributionData {
    fn from(v: AccelerationContribution) -> Self {
        v.inner
    }
}

impl std::ops::Mul<DtFraction> for AccelerationContribution {
    type Output = VelocityContribution;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self.inner {
            AccelerationContributionData::Acceleration { a_ref } => {
                VelocityContributionData::AccelerationDt {
                    factor: 1.,
                    a_ref,
                    dt_fraction: dt_fraction.into(),
                }
                .into()
            }
        }
    }
}
