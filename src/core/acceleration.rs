use super::VectorQuantity;

#[derive(Clone, Copy, Debug)]
pub struct Unit;

pub type Acceleration = VectorQuantity<Unit, super::Velocity>;
