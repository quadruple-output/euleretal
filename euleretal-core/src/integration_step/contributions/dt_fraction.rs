use crate::{Duration, Fraction};

pub trait FractionSpec: Copy + ::std::ops::Mul<Duration, Output = Duration> {}

impl FractionSpec for Fraction {}
impl<const N: usize, const D: usize> FractionSpec for DtFraction<N, D> {}

#[derive(Clone, Copy, Debug)]
pub struct DtFraction<const NUMERATOR: usize, const DENOMINATOR: usize>;

impl<const N: usize, const D: usize> From<DtFraction<N, D>> for f32 {
    fn from(_: DtFraction<N, D>) -> Self {
        #![allow(clippy::cast_precision_loss)]
        N as f32 / D as f32
    }
}

impl<const N: usize, const D: usize> From<DtFraction<N, D>> for Fraction {
    fn from(_dt_fraction: DtFraction<N, D>) -> Self {
        Fraction::new(N, D)
    }
}

impl<const N: usize, const D: usize> ::std::ops::Mul<Duration> for DtFraction<N, D> {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        #![allow(clippy::cast_precision_loss)]
        (N as f32 / D as f32) * rhs
    }
}

impl<const N: usize, const D: usize> DtFraction<N, D> {
    #[allow(clippy::unused_self)]
    #[must_use]
    pub const fn half(self) -> DtFraction<N, { D * 2 }> {
        DtFraction
    }
}
