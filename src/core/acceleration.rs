use super::vector_quantity::VectorQuantity;

#[derive(Clone, Copy)]
pub struct Unit;

pub type Acceleration = VectorQuantity<Unit, super::velocity::Velocity>;
