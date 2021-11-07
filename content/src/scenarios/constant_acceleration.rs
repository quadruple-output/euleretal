use super::core::{Acceleration, AccelerationField, Position};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ConstantAcceleration;

#[cfg_attr(feature = "persistence", typetag::serde)]
impl AccelerationField for ConstantAcceleration {
    fn value_at(&self, _pos: Position) -> Acceleration {
        Acceleration::new(0., -1., 0.)
    }

    fn label(&self) -> String {
        "Constant Acceleration".to_string()
    }
}
