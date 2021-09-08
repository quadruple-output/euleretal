use super::VectorQuantity;

#[derive(Clone, Copy)]
pub struct Unit;

pub type Acceleration = VectorQuantity<Unit, super::Velocity>;
