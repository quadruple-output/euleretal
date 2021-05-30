use super::{
    core::{
        AccelerationField, FinalizedCalibrationPoints, Fraction, Integrator, NewSampleWithPoints,
        OneStepWithCalibrationPoints, Samples, StartCondition,
    },
    import::R32,
};

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl Integrator for Euler {
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
        <Self as OneStepWithCalibrationPoints<2>>::integrate(
            acceleration_field,
            start_condition,
            num_steps,
            dt,
        )
    }
}

impl OneStepWithCalibrationPoints<2> for Euler {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSampleWithPoints<2>,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let mid_point_fraction = fraction!(1 / 2);
        let mid_point_dt = mid_point_fraction * dt;

        let mid_point_position = current.position
            + (current.velocity + current.acceleration * mid_point_dt) * mid_point_dt;
        let mid_point_acceleration = acceleration_field.value_at(mid_point_position);

        next.calibration_points[0].position = current.position;
        next.calibration_points[0].dt_fraction = fraction!(0 / 2);
        next.calibration_points[0].velocity = Some(current.velocity);
        next.calibration_points[0].eff_velocity = Some(current.velocity * dt);

        next.calibration_points[1].position = mid_point_position;
        next.calibration_points[1].dt_fraction = mid_point_fraction;
        next.calibration_points[1].acceleration = Some(mid_point_acceleration);
        next.calibration_points[1].eff_acceleration = Some(mid_point_acceleration * dt * dt);

        // this cannot be generically computed from the calib.points (yet! → todo):
        next.velocity = next.calibration_points[0].velocity.unwrap()
            + next.calibration_points[1].acceleration.unwrap() * dt;
        // this is always the sum of all `eff_` values off all calib.points:
        next.position = current.position
            + next.calibration_points[0].eff_velocity.unwrap()
            + next.calibration_points[1].eff_acceleration.unwrap();
    }
}

pub struct SecondOrder {}

impl SecondOrder {
    pub fn new() -> Self {
        SecondOrder {}
    }
}

impl Integrator for SecondOrder {
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
        <Self as OneStepWithCalibrationPoints<2>>::integrate(
            acceleration_field,
            start_condition,
            num_steps,
            dt,
        )
    }
}

impl OneStepWithCalibrationPoints<2> for SecondOrder {
    fn integrate_step(
        current: &StartCondition,
        next: &mut NewSampleWithPoints<2>,
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

        next.calibration_points[0].position = current.position;
        next.calibration_points[0].dt_fraction = fraction!(0 / 2);
        next.calibration_points[0].velocity = Some(current.velocity);
        next.calibration_points[0].eff_velocity = Some(current.velocity * dt);

        next.calibration_points[1].position = mid_point_position;
        next.calibration_points[1].dt_fraction = mid_point_fraction;
        next.calibration_points[1].acceleration = Some(mid_point_acceleration);
        next.calibration_points[1].eff_acceleration = Some(0.5 * mid_point_acceleration * dt * dt);

        next.velocity = next.calibration_points[0].velocity.unwrap()
            + next.calibration_points[1].acceleration.unwrap() * dt;
        next.position = current.position
            + next.calibration_points[0].eff_velocity.unwrap()
            + next.calibration_points[1].eff_acceleration.unwrap();
    }
}
