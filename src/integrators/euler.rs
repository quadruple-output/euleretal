use crate::core::NewSampleWithPoints;

use super::core::{AccelerationField, Integrator, StartCondition};

pub struct Broken {}

impl Broken {
    pub fn new() -> Self {
        Broken {}
    }
}

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
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
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
        "v' = v + a dt\n\
         s' = s + v' dt\n\
             = s + v dt + a dt²" // !! this string contains non-breaking spaces
            .to_string()
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
        dt: f32,
        _acceleration_field: &dyn AccelerationField,
    ) {
        next.velocity = current.velocity + current.acceleration * dt;
        next.position = current.position + next.velocity * dt;
    }
}
