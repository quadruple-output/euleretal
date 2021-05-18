use crate::core::integrator::OneStepWithCalibrationPoints;
use crate::core::samples::{FinalizedCalibrationPoints, NewSampleWithPoints, StartCondition};
use crate::prelude::*;

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl core::Integrator for Euler {
    fn label(&self) -> String {
        "Midpoint (explicit, Euler)".to_string()
    }

    fn description(&self) -> String {
        "v₁ = v + a ½dt\n\
         s₁ = s + v₁ ½dt\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v' dt\n\
             = s + v dt + a₁ dt²" // !! string contains non-breakable spaces
            .to_string()
    }

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
        <Self as OneStepWithCalibrationPoints<1>>::integrate(
            acceleration_field,
            start_condition,
            num_steps,
            dt,
        )
    }
}

impl OneStepWithCalibrationPoints<1> for Euler {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSampleWithPoints<1>,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let mid_point_fraction = fraction!(1 / 2);
        let mid_point_dt = mid_point_fraction * dt;

        let mid_point_position = current.position
            + (current.velocity + current.acceleration * mid_point_dt) * mid_point_dt;
        let mid_point_acceleration = acceleration_field.value_at(mid_point_position);

        next.velocity = current.velocity + mid_point_acceleration * dt;
        next.position = current.position + next.velocity * dt;

        next.calibration_points[0].dt_fraction = mid_point_fraction;
        next.calibration_points[0].acceleration = mid_point_acceleration;
        next.calibration_points[0].position = mid_point_position;
    }
}

pub struct SecondOrder {}

impl SecondOrder {
    pub fn new() -> Self {
        SecondOrder {}
    }
}

impl core::Integrator for SecondOrder {
    fn label(&self) -> String {
        "Midpoint (explicit, SecondOrder)".to_string()
    }

    fn description(&self) -> String {
        "s₁ = s + v ½dt + ½ a (½dt)²\n\
         a₁ = a(s₁)\n\
         v' = v + a₁ dt\n\
         s' = s + v dt + ½ a₁ dt²" // !! string contains non-breakable spaces
            .to_string()
    }

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples<FinalizedCalibrationPoints> {
        <Self as OneStepWithCalibrationPoints<1>>::integrate(
            acceleration_field,
            start_condition,
            num_steps,
            dt,
        )
    }
}

impl OneStepWithCalibrationPoints<1> for SecondOrder {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSampleWithPoints<1>,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let mid_point_fraction = fraction!(1 / 2);
        let mid_point_dt = mid_point_fraction * dt;

        let mid_point_position = current.position
            + current.velocity * mid_point_dt
            + 0.5 * current.acceleration * mid_point_dt * mid_point_dt;
        let mid_point_acceleration = acceleration_field.value_at(mid_point_position);

        next.velocity = current.velocity + mid_point_acceleration * dt;
        next.position =
            current.position + current.velocity * dt + 0.5 * mid_point_acceleration * dt * dt;

        next.calibration_points[0].dt_fraction = mid_point_fraction;
        next.calibration_points[0].acceleration = mid_point_acceleration;
        next.calibration_points[0].position = mid_point_position;
    }
}
