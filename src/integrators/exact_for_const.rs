use super::core::{AccelerationField, Integrator, NewSampleWithPoints, StartCondition};

pub struct ExactForConst {}

impl ExactForConst {
    pub fn new() -> Self {
        ExactForConst {}
    }
}

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
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
        dt: f32,
        _acceleration_field: &dyn AccelerationField,
    ) {
        next.velocity = current.velocity + current.acceleration * dt;
        next.position =
            current.position + current.velocity * dt + 0.5 * current.acceleration * dt * dt;
    }
}
