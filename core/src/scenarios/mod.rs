mod center_mass;
mod constant_acceleration;

pub use center_mass::CenterMass;
pub use constant_acceleration::ConstantAcceleration;

/// Use this mod in `#[serde(with="<path_to_this_mod>")]` if you need to serialize an attribute of
/// type `Box<dyn AccelerationField>`
pub mod serde_box_dyn_acceleration_field {
    use super::{CenterMass, ConstantAcceleration};
    use crate::AccelerationField;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Deserialize, Serialize)]
    pub enum AccelerationFieldSerDe {
        CenterMass(#[serde(skip)] CenterMass),
        ConstantAcceleration(#[serde(skip)] ConstantAcceleration),
    }

    #[allow(clippy::borrowed_box)]
    #[allow(clippy::missing_errors_doc)]
    pub fn serialize<S>(
        acceleration_field: &Box<dyn AccelerationField>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        acceleration_field.to_concrete_type().serialize(serializer)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<dyn AccelerationField>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match AccelerationFieldSerDe::deserialize(deserializer)? {
            AccelerationFieldSerDe::CenterMass(accel) => {
                Box::new(accel) as Box<dyn AccelerationField>
            }
            AccelerationFieldSerDe::ConstantAcceleration(accel) => {
                Box::new(accel) as Box<dyn AccelerationField>
            }
        })
    }
}
