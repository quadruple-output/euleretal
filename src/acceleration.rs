use bevy::math::Vec3;

pub trait Acceleration {
    fn value_at(_pos: Vec3) -> Vec3 {
        Default::default()
    }
}
