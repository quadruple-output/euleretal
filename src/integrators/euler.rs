use crate::core::integrator::ZeroKnowledge;
use crate::core::samples::{FinalizedCalibrationPoints, NewSample, StartCondition};
use crate::prelude::*;

pub struct Deferred {}

impl Deferred {
    pub fn new() -> Self {
        Deferred {}
    }
}

impl Integrator for Deferred {
    fn label(&self) -> String {
        "Deferred Euler".to_string()
    }

    fn description(&self) -> String {
        "s' = s + v * dt\n
         v' = v + a * dt"
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

impl ZeroKnowledge for Deferred {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSample,
        dt: f32,
        _acceleration_field: &dyn AccelerationField,
    ) {
        next.position = current.position + current.velocity * dt;
        next.velocity = current.velocity + current.acceleration * dt;
    }
}

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl Integrator for Euler {
    fn label(&self) -> String {
        "Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a * dt\n
         s' = s + v' * dt"
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
        _acceleration_field: &dyn AccelerationField,
    ) {
        next.velocity = current.velocity + current.acceleration * dt;
        next.position = current.position + next.velocity * dt;
    }
}
