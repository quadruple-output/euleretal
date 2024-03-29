use super::step::PositionRef;

#[derive(Clone)]
pub struct Acceleration {
    pub(in crate::integration_step) a: crate::Acceleration,
    pub(in crate::integration_step) sampling_position: PositionRef,
}
