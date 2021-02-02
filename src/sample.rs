use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct Sample {
    /// Position
    pub s: Vec3,
    /// Velocity
    pub v: Vec3,
    /// Time
    pub t: f32,
}

impl From<(Vec3, Vec3, f32)> for Sample {
    fn from(tuple: (Vec3, Vec3, f32)) -> Self {
        Self {
            s: tuple.0,
            v: tuple.1,
            t: tuple.2,
        }
    }
}
