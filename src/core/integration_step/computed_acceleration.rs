use super::core::Acceleration;
use super::step::PositionRef;

pub struct ComputedAcceleration {
    pub(super) a: Acceleration,
    pub(super) sampling_position: PositionRef,
}

impl ::std::ops::Mul<&ComputedAcceleration> for f32 {
    type Output = Acceleration;

    fn mul(self, rhs: &ComputedAcceleration) -> Self::Output {
        self * rhs.a
    }
}
