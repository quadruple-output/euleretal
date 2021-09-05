use super::{
    import::{Vec3, R32},
    Fraction,
};
use ::std::{
    hash::Hash,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Duration(pub R32);

impl Hash for Duration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl Mul<f32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<R32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: R32) -> Self::Output {
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

impl From<Duration> for R32 {
    fn from(d: Duration) -> Self {
        d.0
    }
}

impl Mul<Fraction> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Fraction) -> Self::Output {
        self * rhs.to_f32()
    }
}

impl Mul<Duration> for f32 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Self::Output {
        duration * self
    }
}

impl Mul<Duration> for R32 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Self::Output {
        duration * self
    }
}

impl Mul<Duration> for Fraction {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Self::Output {
        duration * self
    }
}
