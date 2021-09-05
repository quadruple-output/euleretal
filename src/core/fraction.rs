use super::import::{Vec3, R32};
use ::std::ops::Mul;

#[derive(Clone, Copy, PartialEq)]
pub struct Fraction {
    numerator: usize,
    denominator: usize,
}

macro_rules! fraction(
($numerator:literal / $denominator:literal) => {crate::core::Fraction::new($numerator,$denominator)}
);

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
        f.numerator as f32 / f.denominator as f32
    }
}

impl From<Fraction> for R32 {
    fn from(f: Fraction) -> Self {
        R32::new(f.into()).unwrap()
    }
}

impl Mul<Fraction> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Fraction) -> Vec3 {
        self * f32::from(rhs)
    }
}
