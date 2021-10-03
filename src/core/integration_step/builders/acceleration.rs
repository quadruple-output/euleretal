use super::{
    integration_step::{contributions, step::AccelerationRef},
    velocity::VelocityContribution,
    DtFraction,
};

#[derive(Clone, Copy)]
pub struct AccelerationContribution {
    inner: contributions::acceleration::Variant,
}

impl From<contributions::acceleration::Variant> for AccelerationContribution {
    fn from(data: contributions::acceleration::Variant) -> Self {
        Self { inner: data }
    }
}

impl From<AccelerationRef> for AccelerationContribution {
    fn from(a_ref: AccelerationRef) -> Self {
        contributions::acceleration::Variant::Acceleration { factor: 1., a_ref }.into()
    }
}

impl From<AccelerationContribution> for contributions::acceleration::Variant {
    fn from(v: AccelerationContribution) -> Self {
        v.inner
    }
}

impl std::ops::Mul<DtFraction> for AccelerationContribution {
    type Output = VelocityContribution;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self.inner {
            contributions::acceleration::Variant::Acceleration { factor, a_ref } => {
                contributions::velocity::Variant::AccelerationDt {
                    factor,
                    a_ref,
                    dt_fraction: dt_fraction.into(),
                }
                .into()
            }
        }
    }
}

impl std::ops::Mul<AccelerationContribution> for f32 {
    type Output = AccelerationContribution;

    fn mul(self, rhs: AccelerationContribution) -> Self::Output {
        match rhs.inner {
            contributions::acceleration::Variant::Acceleration { factor, a_ref } => {
                contributions::acceleration::Variant::Acceleration {
                    factor: self * factor,
                    a_ref,
                }
            }
        }
        .into()
    }
}
