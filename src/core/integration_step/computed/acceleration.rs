use super::core;
use super::step::PositionRef;

pub struct Acceleration {
    pub(in crate::core::integration_step) a: core::Acceleration,
    pub(in crate::core::integration_step) sampling_position: PositionRef,
}

impl ::std::ops::Mul<&Acceleration> for f32 {
    type Output = core::Acceleration;

    fn mul(self, rhs: &Acceleration) -> Self::Output {
        self * rhs.a
    }
}
