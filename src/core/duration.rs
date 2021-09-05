use super::{
    import::{Vec3, R32},
    Fraction,
};
use ::std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Duration(pub R32); // todo: make self.0 private and use into() for instantiation

impl From<f32> for Duration {
    fn from(n: f32) -> Self {
        Self(R32::new(n).unwrap())
    }
}

impl From<R32> for Duration {
    fn from(n: R32) -> Self {
        Self(n)
    }
}

impl From<Duration> for R32 {
    fn from(d: Duration) -> Self {
        d.0
    }
}

impl From<Duration> for f32 {
    fn from(d: Duration) -> Self {
        d.0.into_inner()
    }
}

impl Add<Duration> for Duration {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl Mul<f32> for Duration {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<R32> for Duration {
    type Output = Self;

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

impl Mul<Fraction> for Duration {
    type Output = Self;

    fn mul(self, rhs: Fraction) -> Self::Output {
        self * f32::from(rhs)
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

impl Div<Duration> for Duration {
    type Output = R32;

    fn div(self, rhs: Duration) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<f32> for Duration {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<R32> for Duration {
    type Output = Self;

    fn div(self, rhs: R32) -> Self::Output {
        Self(self.0 / rhs)
    }
}
