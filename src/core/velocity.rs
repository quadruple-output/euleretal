use super::vector_quantity::VectorQuantity;

#[derive(Clone, Copy)]
pub struct Unit;

pub type Velocity = VectorQuantity<Unit, super::position::Translation>;
