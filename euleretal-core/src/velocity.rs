use super::VectorQuantity;

#[derive(Clone, Copy, Debug)]
pub struct Unit;

pub type Velocity = VectorQuantity<Unit, super::Move>;
