use super::import::OrderedF32;
use ::std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    ::serde::Deserialize,
    ::serde::Serialize,
)]
#[serde(transparent)]
pub struct Duration {
    inner: OrderedF32,
}

impl From<f32> for Duration {
    fn from(n: f32) -> Self {
        Self { inner: n.into() }
    }
}

impl From<Duration> for f32 {
    fn from(d: Duration) -> Self {
        d.inner.into()
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration {
            inner: self.inner + rhs.inner,
        }
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul<f32> for Duration {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner * rhs,
        }
    }
}

impl Mul<Duration> for f32 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Self::Output {
        duration * self
    }
}

impl Div for Duration {
    type Output = f32;

    fn div(self, rhs: Duration) -> Self::Output {
        self.inner.into_inner() / rhs.inner.into_inner()
    }
}

impl Div<f32> for Duration {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner / rhs,
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
