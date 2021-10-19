use super::import::{OrderedF32, Vec3};
use ::std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Duration(OrderedF32);

impl From<f32> for Duration {
    fn from(n: f32) -> Self {
        Self(n.into())
    }
}

impl From<Duration> for f32 {
    fn from(d: Duration) -> Self {
        d.0.into()
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

impl Mul<Duration> for f32 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Self::Output {
        duration * self
    }
}

impl Div<Duration> for Duration {
    type Output = f32;

    fn div(self, rhs: Duration) -> Self::Output {
        self.0.into_inner() / rhs.0.into_inner()
    }
}

impl Div<f32> for Duration {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
