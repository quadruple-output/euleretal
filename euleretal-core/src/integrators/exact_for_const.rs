use crate::{
    integration_step::builders::{self, Collector},
    Integrator,
};

#[derive(Clone, Copy, Debug, Default, ::serde::Deserialize, ::serde::Serialize)]
pub struct ExactForConst;

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

    fn to_concrete_type(&self) -> crate::integrators::serde_box_dyn_integrator::IntegratorSerDe {
        crate::integrators::serde_box_dyn_integrator::IntegratorSerDe::ExactForConst(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integrators::test_util::TestSetup;

    #[test]
    fn exact_for_const() {
        let ctx = TestSetup::default();
        ctx.assert_first_step(&ExactForConst, |s0, v0, a0, _a, dt| {
            let s1 = s0 + v0 * dt + 0.5 * a0 * dt * dt;
            let v1 = v0 + a0 * dt;
            (s1, v1)
        });
    }
}
