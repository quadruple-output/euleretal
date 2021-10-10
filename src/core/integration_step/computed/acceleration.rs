use super::{core, step::PositionRef};

pub struct Acceleration {
    pub(in crate::core::integration_step) a: core::Acceleration,
    pub(in crate::core::integration_step) sampling_position: PositionRef,
}
