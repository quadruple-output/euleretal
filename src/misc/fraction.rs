use crate::prelude::*;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Fraction {
    numerator: usize,
    denominator: usize,
    as_float: f32,
}

macro_rules! fraction(
($numerator:literal / $denominator:literal) => {Fraction::new($numerator,$denominator)}
);

impl Fraction {
    pub fn new(numerator: usize, denominator: usize) -> Self {
        Self {
            numerator,
            denominator,
            as_float: numerator as f32 / denominator as f32,
        }
    }

    pub fn to_f32(&self) -> f32 {
        self.as_float
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
            as_float: 1.,
        }
    }
}

impl Mul<Fraction> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Fraction) -> Vec3 {
        self * rhs.to_f32()
    }
}

impl Mul<f32> for Fraction {
    type Output = f32;

    fn mul(self, rhs: f32) -> f32 {
        self.as_float * rhs
    }
}
