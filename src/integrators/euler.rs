use crate::core::{integrator::Sample, samples::FinalizedCalibrationPoints};
use crate::prelude::*;

#[derive(Debug)]
pub struct Implicit;

impl Integrator for Implicit {
    fn label(&self) -> String {
        "Implicit Euler".to_string()
    }

    fn integrate_step(
        &mut self,
        acceleration_field: &dyn AccelerationField,
        start: &Sample,
        dt: R32,
    ) -> Sample {
        let s0 = start.s;
        let v0 = start.v;
        let a0 = start.a;
        let v1 = v0 + a0 * dt.into_inner();
        let s1 = s0 + v1 * dt.into_inner();
        Sample {
            s: s1,
            v: v1,
            a: acceleration_field.value_at(s1),
        }
    }

    fn samples(self) -> Samples<FinalizedCalibrationPoints> {}
}
