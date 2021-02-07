use bevy::math::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    /// Step Number
    pub n: usize,
    /// Time
    pub t: f32,
    /// delta t:
    pub dt: f32,
    /// Position
    pub s: Vec3,
    /// Velocity
    pub v: Vec3,
    /// Acceleration
    pub a: Vec3,
}

impl From<(usize, f32, f32, Vec3, Vec3, Vec3)> for Sample {
    fn from(tuple: (usize, f32, f32, Vec3, Vec3, Vec3)) -> Self {
        Self {
            n: tuple.0,
            t: tuple.1,
            dt: tuple.2,
            s: tuple.3,
            v: tuple.4,
            a: tuple.5,
        }
    }
}
