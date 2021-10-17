use super::{step::AccelerationRef, velocity, DtFraction};

#[derive(Clone, Copy)]
pub enum Variant {
    Acceleration { factor: f32, a_ref: AccelerationRef },
}

impl From<AccelerationRef> for Variant {
    fn from(a_ref: AccelerationRef) -> Self {
        Self::Acceleration { factor: 1., a_ref }
    }
}

impl<const N: usize, const D: usize> std::ops::Mul<DtFraction<N, D>> for Variant {
    type Output = velocity::Variant<N, D>;

    fn mul(self, dt_fraction: DtFraction<N, D>) -> Self::Output {
        match self {
            Self::Acceleration { factor, a_ref } => velocity::Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            },
        }
    }
}

impl std::ops::Mul<Variant> for f32 {
    type Output = Variant;

    fn mul(self, rhs: Variant) -> Self::Output {
        match rhs {
            Variant::Acceleration { factor, a_ref } => Variant::Acceleration {
                factor: self * factor,
                a_ref,
            },
        }
    }
}
