use crate::core::integrator::ZeroKnowledge;
use crate::core::samples::{FinalizedCalibrationPoints, NewSample, StartCondition};
use crate::prelude::*;

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl Integrator for Euler {
    fn label(&self) -> String {
        "Mid Point (Euler)".to_string()
    }

    fn description(&self) -> String {
        "v₁ = v + a ½dt\n\
         s₁ = s + v₁ ½dt\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v' dt\n\
             = s + v dt + a₁ dt²" // !! string contains non-breakable spaces
            .to_string()
    }

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
        <Self as ZeroKnowledge>::integrate(acceleration_field, start_condition, num_steps, dt)
    }
}

impl ZeroKnowledge for Euler {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSample,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let mid_point_position =
            current.position + current.velocity * 0.5 * dt + current.acceleration * 0.25 * dt * dt;
        let mid_point_acceleration = acceleration_field.value_at(mid_point_position);
        next.velocity = current.velocity + mid_point_acceleration * dt;
        next.position = current.position + next.velocity * dt;
    }
}

pub struct SecondOrder {}

impl SecondOrder {
    pub fn new() -> Self {
        SecondOrder {}
    }
}

impl Integrator for SecondOrder {
    fn label(&self) -> String {
        "Mid Point (SecondOrder)".to_string()
    }

    fn description(&self) -> String {
        "s₁ = s + v ½dt + ½ a (½dt)²\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v dt + ½ a₁ dt²" // !! string contains non-breakable spaces
            .to_string()
    }

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
        <Self as ZeroKnowledge>::integrate(acceleration_field, start_condition, num_steps, dt)
    }
}

impl ZeroKnowledge for SecondOrder {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSample,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let mid_point_position = current.position
            + current.velocity * 0.5 * dt
            + 0.5 * current.acceleration * 0.25 * dt * dt;
        let mid_point_acceleration = acceleration_field.value_at(mid_point_position);
        next.velocity = current.velocity + mid_point_acceleration * dt;
        next.position =
            current.position + current.velocity * dt + 0.5 * mid_point_acceleration * dt * dt;
    }
}
