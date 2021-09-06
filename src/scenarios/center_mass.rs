use super::core::{Acceleration, AccelerationField, Position};

#[derive(Clone, Copy)]
pub struct CenterMass;

impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Position) -> Acceleration {
        let distance_squared = pos.coords.norm_squared();
        if distance_squared > f32::EPSILON {
            let distance_squared_recip = distance_squared.recip();
            let distance_recip = distance_squared_recip.sqrt();
            let direction_normalized = -pos.coords * distance_recip;
            direction_normalized * distance_squared_recip
        } else {
            Acceleration::zeros()
        }
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }
}
