use crate::{Acceleration, AccelerationField, Position};

#[derive(Clone, Copy)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct CenterMass;

impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Position) -> Acceleration {
        let distance_squared_recip = pos.as_vector().norm_squared().recip();
        (-pos.as_vector() * distance_squared_recip.sqrt() * distance_squared_recip).into()
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }

    #[cfg(feature = "persistence")]
    fn to_concrete_type(
        &self,
    ) -> crate::scenarios::serde_box_dyn_acceleration_field::AccelerationFieldSerDe {
        crate::scenarios::serde_box_dyn_acceleration_field::AccelerationFieldSerDe::CenterMass(
            *self,
        )
    }
}
