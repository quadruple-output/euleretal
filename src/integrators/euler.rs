use crate::core::integrator::ZeroKnowledge;
use crate::core::samples::{FinalizedCalibrationPoints, NewSample, StartCondition};
use crate::prelude::*;

pub struct Implicit {}

impl Implicit {
    pub fn new() -> Self {
        Implicit {}
    }
}

impl Integrator for Implicit {
    fn label(&self) -> String {
        "Implicit Euler".to_string()
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

impl ZeroKnowledge for Implicit {
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
