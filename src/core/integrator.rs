use super::samples::{self, NewSample, StartCondition, WithoutCalibrationPoints};
use crate::prelude::*;

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<samples::FinalizedCalibrationPoints>;
}

pub trait ZeroKnowledge {
    fn integrate(
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<samples::FinalizedCalibrationPoints> {
        let mut samples = Samples::<WithoutCalibrationPoints>::new(start_condition, num_steps);
        for _ in 0..num_steps {
            let current = samples.current();
            let mut next = NewSample {
                dt,
                ..NewSample::default()
            };

            Self::integrate_step(&current, &mut next, dt.into(), acceleration_field);

            next.acceleration = acceleration_field.value_at(next.position);
            samples.push_sample(&next);
        }
        samples.finalized()
    }

    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSample,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    );
}
