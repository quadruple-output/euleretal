use super::{core::Duration, import::Vec3};
use ::std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction {
    numerator: usize,
    denominator: usize,
}

impl Fraction {
    pub fn new(numerator: usize, denominator: usize) -> Self {
        assert!(denominator != 0);
        Self {
            numerator,
            denominator,
        }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}

impl From<Fraction> for f32 {
    fn from(f: Fraction) -> Self {
        #![allow(clippy::cast_precision_loss)]
        f.numerator as f32 / f.denominator as f32
    }
}

impl Mul<Duration> for Fraction {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        f32::from(self) * rhs
    }
}

impl Mul<Fraction> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Fraction) -> Vec3 {
        self * f32::from(rhs)
    }
}
