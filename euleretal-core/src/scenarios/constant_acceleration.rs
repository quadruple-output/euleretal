use crate::{Acceleration, AccelerationField, Position};

#[derive(Clone, Copy, Default, ::serde::Deserialize, ::serde::Serialize)]
pub struct ConstantAcceleration;

impl AccelerationField for ConstantAcceleration {
    fn value_at(&self, _pos: Position) -> Acceleration {
        Acceleration::new(0., -1., 0.)
    }

    fn label(&self) -> String {
        "Constant Acceleration".to_string()
    }

    fn to_concrete_type(
        &self,
    ) -> crate::scenarios::serde_box_dyn_acceleration_field::AccelerationFieldSerDe {
        crate::scenarios::serde_box_dyn_acceleration_field::AccelerationFieldSerDe::ConstantAcceleration(*self)
    }
}
