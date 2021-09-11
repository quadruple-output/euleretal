use super::{
    import::{Point3, Vec3, R32},
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
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Point3::new(x, y, z).into()
    }

    pub fn origin() -> Position {
        Point3::origin().into()
    }

    pub fn distance_squared(&self, other: Position) -> f32 {
        (self.0 - other.0).norm_squared()
    }

    pub fn as_point(&self) -> &Point3 {
        &self.0
    }

    pub fn as_vector(&self) -> &Vec3 {
        &self.0.coords
    }

    pub fn direction_to(&self, other: Position) -> Move {
        self.vector_to(other).into()
    }

    pub fn vector_to(&self, other: Position) -> Vec3 {
        other.0 - self.0
    }
}

impl ::std::hash::Hash for Position {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        R32::new(self.0.x).unwrap().hash(state);
        R32::new(self.0.y).unwrap().hash(state);
        R32::new(self.0.z).unwrap().hash(state);
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
        //
        // Here: the implementation of Hash only works for the subset of all possible values of f32.
        //       for the other values, the above condition holds (I assume)

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
