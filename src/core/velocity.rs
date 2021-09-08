use super::VectorQuantity;

#[derive(Clone, Copy)]
pub struct Unit;

pub type Velocity = VectorQuantity<Unit, super::Move>;
