use super::core::{Acceleration, AccelerationField, Position};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct CenterMass;

#[cfg_attr(feature = "persistence", typetag::serde)]
impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Position) -> Acceleration {
        let distance_squared_recip = pos.as_vector().norm_squared().recip();
        (-pos.as_vector() * distance_squared_recip.sqrt() * distance_squared_recip).into()
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }
}
