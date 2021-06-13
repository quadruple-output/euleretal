use super::import::Vec3;
use ::std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Fraction {
    numerator: usize,
    denominator: usize,
}

macro_rules! fraction(
($numerator:literal / $denominator:literal) => {Fraction::new($numerator,$denominator)}
);

impl Fraction {
    pub fn new(numerator: usize, denominator: usize) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn to_f32(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
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

impl Mul<Fraction> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Fraction) -> Vec3 {
        self * rhs.to_f32()
    }
}

impl Mul<&Fraction> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Fraction) -> Vec3 {
        self * rhs.to_f32()
    }
}

impl<IntoF32> Mul<IntoF32> for Fraction
where
    IntoF32: Into<f32>,
{
    type Output = f32;

    fn mul(self, rhs: IntoF32) -> f32 {
        self.to_f32() * rhs.into()
    }
}

impl<IntoF32> Mul<IntoF32> for &Fraction
where
    IntoF32: Into<f32>,
{
    type Output = f32;

    fn mul(self, rhs: IntoF32) -> f32 {
        self.to_f32() * rhs.into()
    }
}
