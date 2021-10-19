use super::{
    import::{OrderedF32, Point3, Vec3},
    Move,
};

#[derive(Clone, Copy, Debug)]
pub struct Position(Point3);

impl From<Point3> for Position {
    fn from(p: Point3) -> Self {
        Self(p)
    }
}

impl From<Position> for Point3 {
    fn from(p: Position) -> Self {
        p.0
    }
}

impl From<Position> for Vec3 {
    fn from(p: Position) -> Self {
        p.0.coords
    }
}

impl Position {
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Point3::new(x, y, z).into()
    }

    #[must_use]
    pub fn origin() -> Position {
        Point3::origin().into()
    }

    #[must_use]
    pub fn distance_squared(&self, other: Position) -> f32 {
        (self.0 - other.0).norm_squared()
    }

    #[must_use]
    pub fn as_point(&self) -> &Point3 {
        &self.0
    }

    #[must_use]
    pub fn as_vector(&self) -> &Vec3 {
        &self.0.coords
    }

    #[must_use]
    pub fn direction_to(&self, other: Position) -> Move {
        self.vector_to(other).into()
    }

    #[must_use]
    pub fn vector_to(&self, other: Position) -> Vec3 {
        other.0 - self.0
    }
}

impl ::std::hash::Hash for Position {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        OrderedF32::from(self.0.x).hash(state);
        OrderedF32::from(self.0.y).hash(state);
        OrderedF32::from(self.0.z).hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        // Clippy Lint https://rust-lang.github.io/rust-clippy/master/index.html#derive_hash_xor_eq:
        // ```
        // The implementation of Hash and PartialEq must agree (for example for
        // use with HashMap) so it’s probably a bad idea to use a
        // default-generated Hash implementation with an explicitly defined
        // PartialEq. In particular, the following must hold for any type:
        // `k1 == k2 ⇒ hash(k1) == hash(k2)`
        // ```
        // I believe this is true for this impl.

        self.0 == other.0
    }
}

impl ::std::ops::Add<Move> for Position {
    type Output = Position;
    fn add(self, rhs: Move) -> Self::Output {
        Self(self.0 + Vec3::from(rhs))
    }
}

impl ::std::ops::AddAssign<Move> for Position {
    fn add_assign(&mut self, rhs: Move) {
        self.0 += Vec3::from(rhs);
    }
}
