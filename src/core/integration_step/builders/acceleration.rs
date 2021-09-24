use super::core;

pub struct Acceleration1 {
    inner: core::Acceleration,
}

impl From<core::Acceleration> for Acceleration1 {
    fn from(a: core::Acceleration) -> Self {
        Self { inner: a }
    }
}

impl From<Acceleration1> for core::Acceleration {
    fn from(a: Acceleration1) -> Self {
        a.inner
    }
}
