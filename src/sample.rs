use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct Sample {
    /// Step Number
    pub n: usize,
    /// Time
    pub t: f32,
    /// Position
    pub s: Vec3,
    /// Velocity
    pub v: Vec3,
}

impl From<(usize, f32, Vec3, Vec3)> for Sample {
    fn from(tuple: (usize, f32, Vec3, Vec3)) -> Self {
        Self {
            n: tuple.0,
            t: tuple.1,
            s: tuple.2,
            v: tuple.3,
        }
    }
}
