use super::{core, step::PositionRef};

pub struct Acceleration {
    pub(in super::super) a: core::Acceleration,
    pub(in super::super) sampling_position: PositionRef,
}
