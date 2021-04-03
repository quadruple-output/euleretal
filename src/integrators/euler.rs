use std::mem;

use crate::core::samples::{FinalizedCalibrationPoints, WithoutCalibrationPoints};
use crate::core::{integrator::StartCondition, samples::NewSample};
use crate::prelude::*;

pub struct Implicit {}

struct Stepper {
    samples: Option<Samples<WithoutCalibrationPoints>>,
}

impl Implicit {
    pub fn new() -> Self {
        Implicit {}
    }
}

impl Integrator for Implicit {
    fn label(&self) -> String {
        "Implicit Euler".to_string()
    }

    fn stepper(
        &self,
        start_condition: &StartCondition,
        num_steps: usize,
    ) -> Box<dyn crate::core::integrator::Stepper> {
        box Stepper::new(start_condition, num_steps)
    }
}

impl Stepper {
    fn new(start_condition: &StartCondition, sample_capacity: usize) -> Self {
        Stepper {
            samples: Some(Samples::<WithoutCalibrationPoints>::new(
                start_condition,
                sample_capacity,
            )),
        }
    }
}

impl crate::core::integrator::Stepper for Stepper {
    fn integrate_step(
        &mut self,
        acceleration_field: &dyn AccelerationField,
        start: &StartCondition,
        dt: R32,
    ) -> StartCondition {
        let s0 = start.s;
        let v0 = start.v;
        let a0 = start.a;
        let v1 = v0 + a0 * dt.into_inner();
        let s1 = s0 + v1 * dt.into_inner();
        let a1 = acceleration_field.value_at(s1);
        if let Some(ref mut samples) = self.samples {
            samples.push_sample(&NewSample {
                dt,
                position: s1,
                velocity: v1,
                acceleration: a1,
            });
        }
        StartCondition {
            s: s1,
            v: v1,
            a: a1,
        }
    }

    fn samples(&mut self) -> Samples<FinalizedCalibrationPoints> {
        let samples = mem::take(&mut self.samples);
        samples.unwrap().finalized()
    }
}
