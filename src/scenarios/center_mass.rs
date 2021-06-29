use super::{core::AccelerationField, import::Vec3};

#[derive(Clone, Copy)]
pub struct CenterMass;

impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Vec3) -> Vec3 {
        let distance_squared = pos.norm_squared();
        if distance_squared > f32::EPSILON {
            let distance_squared_recip = pos.norm_squared().recip();
            let distance_recip = distance_squared_recip.sqrt();
            let direction_normalized = -pos * distance_recip;
            direction_normalized * distance_squared_recip
        } else {
            Vec3::zeros()
        }
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }
}
