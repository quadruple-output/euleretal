use super::core::{Acceleration, AccelerationField, Position};

pub struct ConstantAcceleration;

impl AccelerationField for ConstantAcceleration {
    fn value_at(&self, _pos: Position) -> Acceleration {
        Acceleration::new(0., -1., 0.)
    }

    fn label(&self) -> String {
        "Constant Acceleration".to_string()
    }
}
