use super::samples;
use crate::prelude::*;

#[derive(Clone)]
pub struct StartCondition {
    pub s: Position,
    pub v: Velocity,
    pub a: Acceleration,
}

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn stepper(&self, start_condition: &StartCondition, num_steps: usize) -> Box<dyn Stepper>;

    fn execute(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        duration: R32,
        dt: R32,
    ) -> Samples<samples::FinalizedCalibrationPoints> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (duration / dt).into_inner() as usize;
        let mut stepper = self.stepper(start_condition, num_steps);
        let mut start_condition = (*start_condition).clone();
        for _ in 1..=num_steps {
            start_condition = stepper.integrate_step(acceleration_field, &start_condition, dt);
        }
        let samples = stepper.samples();
        assert!(samples.step_points().len() == num_steps + 1);
        samples
    }
}

pub trait Stepper {
    fn integrate_step(
        &mut self,
        acceleration_field: &dyn AccelerationField,
        start: &StartCondition,
        dt: R32,
    ) -> StartCondition;

    fn samples(&mut self) -> Samples<samples::FinalizedCalibrationPoints>;
}
