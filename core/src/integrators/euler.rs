use crate::{
    integration_step::builders::{self, Collector},
    Integrator,
};

#[derive(Clone, Copy, Debug, ::serde::Deserialize, ::serde::Serialize)]
pub struct Broken;

impl Integrator for Broken {
    fn label(&self) -> String {
        "Broken Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v dt"
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
        step.compute(s + v * dt);
        step.compute(v + a * dt);
    }

    fn to_concrete_type(&self) -> crate::integrators::serde_box_dyn_integrator::IntegratorSerDe {
        crate::integrators::serde_box_dyn_integrator::IntegratorSerDe::BrokenEuler(*self)
    }
}

#[derive(Clone, Copy, Debug, ::serde::Deserialize, ::serde::Serialize)]
pub struct Euler;

impl Integrator for Euler {
    fn label(&self) -> String {
        "Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v' dt\n    \
            = s + v dt + a dt²"
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
        let v1 = step.compute(v + a * dt);
        step.compute(s + v1 * dt);
    }

    fn to_concrete_type(&self) -> crate::integrators::serde_box_dyn_integrator::IntegratorSerDe {
        crate::integrators::serde_box_dyn_integrator::IntegratorSerDe::Euler(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integrators::test_util::TestSetup;

    #[test]
    fn euler() {
        let ctx = TestSetup::default();
        ctx.assert_first_step(&Euler, |s0, v0, a0, _a, dt| {
            let v1 = v0 + a0 * dt;
            let s1 = s0 + v1 * dt;
            (s1, v1)
        });
    }

    #[test]
    fn broken_euler() {
        let ctx = TestSetup::default();
        ctx.assert_first_step(&Broken, |s0, v0, a0, _a, dt| {
            let v1 = v0 + a0 * dt;
            let s1 = s0 + v0 * dt;
            (s1, v1)
        });
    }
}
