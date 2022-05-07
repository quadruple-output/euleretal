use super::integration_step::builders;
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, fmt::Debug, hash::Hash};

pub trait Integrator: Debug + Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate_step(
        &self,
        s0: builders::Position,
        v0: builders::Velocity,
        a0: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        builder: &mut builders::Step,
    );

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }

    fn to_concrete_type(&self) -> super::integrators::serde_box_dyn_integrator::IntegratorSerDe;
}
