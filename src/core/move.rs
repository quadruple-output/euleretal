use super::{import::Vec3, Position, VectorQuantity};

#[derive(Clone, Copy)]
pub struct Unit;

pub type Move = VectorQuantity<Unit, ()>;

impl From<Position> for Move {
    fn from(p: Position) -> Self {
        Vec3::from(p).into()
    }
}
