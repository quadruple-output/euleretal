use super::core::{
    integration_step::builders::{self, Collector},
    Integrator,
};

#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ExactForConst;

#[cfg_attr(feature = "persistence", typetag::serde)]
impl Integrator for ExactForConst {
    fn label(&self) -> String {
        "Exact for const. acceleration".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v dt + ½ a dt²"
            .to_string()
    }

    fn integrate_step(
        &self,
        s: builders::Position,
        v: builders::Velocity,
        a: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        step: &mut builders::Step,
    ) {
        step.compute(v + a * dt);
        step.compute(s + v * dt + 0.5 * a * dt * dt);
    }
}
