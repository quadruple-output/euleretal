use super::{
    integration_step::{quantity_contributions, step::AccelerationRef},
    velocity::VelocityContribution,
    DtFraction,
};

#[derive(Clone, Copy)]
pub struct AccelerationContribution {
    inner: quantity_contributions::acceleration::Variant,
}

impl From<quantity_contributions::acceleration::Variant> for AccelerationContribution {
    fn from(data: quantity_contributions::acceleration::Variant) -> Self {
        Self { inner: data }
    }
}

impl From<AccelerationRef> for AccelerationContribution {
    fn from(a_ref: AccelerationRef) -> Self {
        quantity_contributions::acceleration::Variant::Acceleration { factor: 1., a_ref }.into()
    }
}

impl From<AccelerationContribution> for quantity_contributions::acceleration::Variant {
    fn from(v: AccelerationContribution) -> Self {
        v.inner
    }
}

impl std::ops::Mul<DtFraction> for AccelerationContribution {
    type Output = VelocityContribution;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self.inner {
            quantity_contributions::acceleration::Variant::Acceleration { factor, a_ref } => {
                quantity_contributions::velocity::Variant::AccelerationDt {
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
            quantity_contributions::acceleration::Variant::Acceleration { factor, a_ref } => {
                quantity_contributions::acceleration::Variant::Acceleration {
                    factor: self * factor,
                    a_ref,
                }
            }
        }
        .into()
    }
}
