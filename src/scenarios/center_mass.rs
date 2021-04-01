use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct CenterMass;

impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Vec3) -> Vec3 {
        let distance_squared = pos.length_squared();
        if distance_squared > f32::EPSILON {
            let distance_squared_recip = pos.length_squared().recip();
            let distance_recip = distance_squared_recip.sqrt();
            let direction_normalized = -pos * distance_recip;
            direction_normalized * distance_squared_recip
        } else {
            Vec3::zero()
        }
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }
}
