use super::core::{
    derived_quantities::{
        AccelerationContribution, ComputedAcceleration, ComputedPosition, ComputedVelocity,
        PositionContribution, VelocityContribution,
    },
    AccelerationField, Duration, Fraction, IntegrationStep, Integrator, NewSampleWithPoints,
    StartCondition,
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

    fn integrate_step(
        &self,
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    ) {
        let dt = Duration(dt.into());

        let mut step = IntegrationStep::new(self, dt);
        let p0 = step.initial_condition(current);
        let mid_point_pos = step
            .compute_position(fraction!(1 / 2))
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(p0.a, 1.)
            .create();
        let mid_point_acceleration =
            step.compute_acceleration_at(mid_point_pos, acceleration_field);
        let final_pos = step
            .compute_position(fraction!(1 / 1))
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(mid_point_acceleration, 1.)
            .create();
        let final_velocity = step
            .compute_velocity(fraction!(1 / 1), final_pos)
            .based_on(p0.v)
            .add_acceleration_dt(mid_point_acceleration, 1.)
            .create();

        let start_position = ComputedPosition::from(current.position);
        let start_velocity = ComputedVelocity::from(current.velocity);
        let start_acceleration = ComputedAcceleration::from(current.acceleration);

        let mid_point_position = ComputedPosition::from(vec![
            PositionContribution::StartPosition {
                sampling_position: start_position.clone(),
            },
            PositionContribution::VelocityDt {
                sampling_position: start_position.clone(),
                factor: 1_f32.into(),
                v: start_velocity.clone(),
                dt,
                dt_fraction: fraction!(1 / 2),
            },
            PositionContribution::AccelerationDtDt {
                sampling_position: start_position.clone(),
                factor: 1_f32.into(),
                a: start_acceleration,
                dt,
                dt_fraction: fraction!(1 / 2),
            },
        ]);
        let mid_point_acceleration =
            ComputedAcceleration::from(vec![AccelerationContribution::Acceleration {
                sampling_position: mid_point_position.clone(),
                a: acceleration_field
                    .value_at(mid_point_position.as_position())
                    .into(),
            }]);

        next.velocity = ComputedVelocity::from(vec![
            VelocityContribution::Velocity {
                sampling_position: start_position.clone(),
                v: start_velocity.clone(),
            },
            VelocityContribution::AccelerationDt {
                sampling_position: mid_point_position.clone(),
                factor: 1_f32.into(),
                a: mid_point_acceleration.clone(),
                dt,
                dt_fraction: fraction!(1 / 1),
            },
        ]);

        next.position = ComputedPosition::from(vec![
            PositionContribution::StartPosition {
                sampling_position: start_position.clone(),
            },
            PositionContribution::VelocityDt {
                sampling_position: start_position,
                factor: 1_f32.into(),
                v: start_velocity,
                dt,
                dt_fraction: fraction!(1 / 1),
            },
            PositionContribution::AccelerationDtDt {
                sampling_position: mid_point_position,
                factor: 1_f32.into(),
                a: mid_point_acceleration,
                dt,
                dt_fraction: fraction!(1 / 1),
            },
        ]);
        // let p0 = current.tracker();

        // let dt_mid_point = fraction!(1 / 2) * dt;
        // let s_mid = p0.s + p0.v * dt_mid_point + 0.5 * p0.a * dt_mid_point * dt_mid_point;
        // let a_mid = s_mid.compute_acceleration(acceleration_field);

        // let v = p0.v + a_mid * dt;
        // let s = p0.s + p0.v * dt + a_mid * dt * dt;
        // s1 | v1;
    }

    fn expected_accelerations_for_step(&self) -> usize {
        2
    }

    fn expected_positions_for_step(&self) -> usize {
        2
    }

    fn expected_velocities_for_step(&self) -> usize {
        2
    }
}

pub struct SecondOrder {}

impl SecondOrder {
    pub fn new() -> Self {
        SecondOrder {}
    }
}

/*
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

    fn integrate_step(
        &self,
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
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

        next.calibration_points.push(CalibrationPoint {
            position: current.position,
            dt_fraction: fraction!(0 / 2),
            velocity: Some(current.velocity),
            eff_velocity: Some(current.velocity * dt),
            acceleration: None,
            eff_acceleration: None,
        });

        next.calibration_points.push(CalibrationPoint {
            position: mid_point_position,
            dt_fraction: mid_point_fraction,
            acceleration: Some(mid_point_acceleration),
            eff_acceleration: Some(0.5 * mid_point_acceleration * dt * dt),
            velocity: None,
            eff_velocity: None,
        });

        next.velocity = next.calibration_points[0].velocity.unwrap()
            + next.calibration_points[1].acceleration.unwrap() * dt;
        next.position = current.position
            + next.calibration_points[0].eff_velocity.unwrap()
            + next.calibration_points[1].eff_acceleration.unwrap();

        // let p0 = current.tracker();

        // let dt_mid_point = fraction!(1 / 2) * dt;
        // let s_mid = p0.s + p0.v * dt_mid_point + 0.5 * p0.a * dt_mid_point * dt_mid_point;
        // let a_mid = s_mid.compute_acceleration(acceleration_field);

        // let v = p0.v + a_mid * dt;
        // let s = p0.s + p0.v * dt + 0.5 * a_mid * dt * dt;
        // s1 | v1;
    }
}
*/
