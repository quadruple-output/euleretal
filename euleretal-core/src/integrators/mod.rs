pub mod euler;
pub mod exact_for_const;
pub mod mid_point;
#[cfg(test)]
mod test_util;

/// Use this mod in `#[serde(with="<path_to_this_mod>")]` if you need to serialize an attribute of
/// type `Box<dyn Integrator>`
pub mod serde_box_dyn_integrator {
    use super::{euler, exact_for_const, mid_point};
    use crate::Integrator;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Deserialize, Serialize)]
    pub enum IntegratorSerDe {
        BrokenEuler(#[serde(skip)] euler::Broken),
        Euler(#[serde(skip)] euler::Euler),
        ExactForConst(#[serde(skip)] exact_for_const::ExactForConst),
        MidPointEuler(#[serde(skip)] mid_point::Euler),
        MidPointSecondOrder(#[serde(skip)] mid_point::SecondOrder),
    }

    #[allow(clippy::borrowed_box)]
    #[allow(clippy::missing_errors_doc)]
    pub fn serialize<S>(integrator: &Box<dyn Integrator>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        integrator.to_concrete_type().serialize(serializer)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<dyn Integrator>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match IntegratorSerDe::deserialize(deserializer)? {
            IntegratorSerDe::BrokenEuler(integrator) => Box::new(integrator) as Box<dyn Integrator>,
            IntegratorSerDe::Euler(integrator) => Box::new(integrator) as Box<dyn Integrator>,
            IntegratorSerDe::ExactForConst(integrator) => {
                Box::new(integrator) as Box<dyn Integrator>
            }
            IntegratorSerDe::MidPointEuler(integrator) => {
                Box::new(integrator) as Box<dyn Integrator>
            }
            IntegratorSerDe::MidPointSecondOrder(integrator) => {
                Box::new(integrator) as Box<dyn Integrator>
            }
        })
    }
}
