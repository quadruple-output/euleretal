use crate::acceleration::Acceleration;
use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct CenterMass;

impl Acceleration for CenterMass {
    fn value_at(&self, pos: Vec3) -> Option<Vec3> {
        let distance_squared = pos.length_squared();
        if distance_squared > f32::EPSILON {
            let distance_squared_recip = pos.length_squared().recip();
            let distance_recip = distance_squared_recip.sqrt();
            let direction_normalized = -pos * distance_recip;
            Some(direction_normalized * distance_squared_recip)
        } else {
            None
        }
    }
}
