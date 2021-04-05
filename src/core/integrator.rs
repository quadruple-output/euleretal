use super::samples::{
    CalibrationPoint, FinalizedCalibrationPoints, NewSample, NewSampleWithPoints, StartCondition,
    WithCalibrationPoints, WithoutCalibrationPoints,
};
use crate::prelude::*;

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints>;
}

pub trait OneStepDirect {
    fn integrate(
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
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

pub trait OneStepWithCalibrationPoints<const N: usize>
where
    [CalibrationPoint; N]: Default,
{
    fn integrate(
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
        let mut samples = Samples::<WithCalibrationPoints<N>>::new(start_condition, num_steps);
        for _ in 0..num_steps {
            let current = samples.current();
            let mut next = NewSampleWithPoints {
                dt,
                ..NewSampleWithPoints::default()
            };

            Self::integrate_step(&current, &mut next, dt.into(), acceleration_field);

            next.acceleration = acceleration_field.value_at(next.position);
            samples.push_sample(&next);
        }
        samples.finalized()
    }

    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSampleWithPoints<N>,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    );
}
