use super::{
    import::{Vec3, R32},
    Fraction,
};
use ::std::{
    hash::Hash,
    ops::{Deref, Mul},
};

#[derive(Clone, Copy)]
pub struct Duration(pub R32);

impl Deref for Duration {
    type Target = R32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for Duration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Mul<f32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Vec3> for Duration {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0.into_inner() * rhs
    }
}

impl Mul<Duration> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Duration) -> Self::Output {
        self * rhs.0.into_inner()
    }
}

impl Mul<&Duration> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Duration) -> Self::Output {
        self * rhs.0.into_inner()
    }
}

impl From<Duration> for f32 {
    fn from(d: Duration) -> Self {
        d.0.into_inner()
    }
}

impl Mul<Fraction> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Fraction) -> Self::Output {
        self * rhs.to_f32()
    }
}

impl Mul<&Fraction> for &Duration {
    type Output = Duration;

    fn mul(self, rhs: &Fraction) -> Self::Output {
        *self * rhs.to_f32()
    }
}
