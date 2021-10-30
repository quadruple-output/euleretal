use super::core::{
    integration_step::builders::{self, Collector},
    Integrator,
};

pub struct Euler;

impl Integrator for Euler {
    fn label(&self) -> String {
        "Midpoint (explicit, Euler)".to_string()
    }

    fn description(&self) -> String {
        "v₁ = v + a ½dt\n\
         s₁ = s + v₁ ½dt\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v' dt\n    \
            = s + v dt + a₁ dt²"
            .to_string()
    }

    fn integrate_step(
        &self,
        s0: builders::Position,
        v0: builders::Velocity,
        a0: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        step: &mut builders::Step,
    ) {
        let dt_mid = dt.half();
        let v_mid = step.compute(v0 + a0 * dt_mid);
        let s_mid = step.compute(s0 + v_mid * dt_mid);
        step.set_display_position(v_mid, s_mid);
        let a_mid = step.acceleration_at(s_mid);
        let v1 = step.compute(v0 + a_mid * dt);
        step.compute(s0 + v1 * dt);
    }
}

pub struct SecondOrder;

impl Integrator for SecondOrder {
    fn label(&self) -> String {
        "Midpoint (explicit, SecondOrder)".to_string()
    }

    fn description(&self) -> String {
        "s₁ = s + v ½dt + ½ a (½dt)²\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v dt + ½ a₁ dt²" // !! string contains non-breakable spaces
            .to_string()
    }

    fn integrate_step(
        &self,
        s0: builders::Position,
        v0: builders::Velocity,
        a0: builders::Acceleration,
        dt: builders::DtFraction<1, 1>,
        step: &mut builders::Step,
    ) {
        let dt_mid = dt.half();
        let s_mid = step.compute(s0 + v0 * dt_mid + 0.5 * a0 * dt_mid * dt_mid);
        let a_mid = step.acceleration_at(s_mid);
        step.compute(s0 + v0 * dt + 0.5 * a_mid * dt * dt);
        step.compute(v0 + a_mid * dt);
    }
}
