use super::{import::Vec3, Position, VectorQuantity};

#[derive(Clone, Copy)]
pub struct Unit;

pub type Move = VectorQuantity<Unit, ()>;

impl ::std::ops::Add<Move> for Position {
    type Output = Position;
    fn add(self, rhs: Move) -> Self::Output {
        self + Vec3::from(rhs)
    }
}

impl ::std::ops::AddAssign<Move> for Position {
    fn add_assign(&mut self, rhs: Move) {
        *self += Vec3::from(rhs);
    }
}
