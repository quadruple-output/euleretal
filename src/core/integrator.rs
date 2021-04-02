use super::samples;
use crate::prelude::*;

pub struct Sample {
    pub s: Position,
    pub v: Velocity,
    pub a: Acceleration,
}

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate_step(
        &mut self,
        acceleration_field: &dyn AccelerationField,
        start: &Sample,
        dt: R32,
    ) -> Sample;

    fn samples(self) -> Samples<samples::FinalizedCalibrationPoints>;
}

pub fn execute(
    integrator: &dyn Integrator,
    acceleration_field: &dyn AccelerationField,
    start_position: Vec3,
    start_velocity: Vec3,
    duration: R32,
    dt: R32,
) -> Samples<samples::FinalizedCalibrationPoints> {
    #[allow(clippy::cast_sign_loss)]
    let num_steps = (duration / dt).into_inner() as usize;
    let mut sample = Sample {
        s: start_position,
        v: start_velocity,
        a: acceleration_field.value_at(start_position),
    };
    for _ in 1..=num_steps {
        sample = integrator.integrate_step(acceleration_field, &sample, dt);
    }
    integrator.samples()
}
