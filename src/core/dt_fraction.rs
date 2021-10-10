use super::core::{Duration, Fraction};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DtFraction(Fraction);

impl From<Fraction> for DtFraction {
    fn from(fraction: Fraction) -> Self {
        Self(fraction)
    }
}

impl From<DtFraction> for Fraction {
    fn from(dt_fraction: DtFraction) -> Self {
        dt_fraction.0
    }
}

impl ::std::ops::Mul<Duration> for DtFraction {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        f32::from(self.0) * rhs
    }
}
