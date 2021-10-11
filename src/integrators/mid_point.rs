use super::core::{AccelerationField, DtFraction, Duration, Integrator, StartCondition, Step};

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
         s' = s + v' dt\n    \
            = s + v dt + a₁ dt²"
            .to_string()
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        acceleration_field: &dyn AccelerationField,
    ) -> Step {
        let mut step = Step::new_deprecated(self.expected_capacities_for_step(), dt);
        let p0 = step.set_start_condition(current);
        let mid_point_pos = step
            .compute_position(DtFraction::<1, 2>)
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(p0.a, 1.)
            .create();
        let mid_point_acceleration =
            step.compute_acceleration_at(mid_point_pos, acceleration_field);
        let final_pos = step
            .compute_position(DtFraction::<1, 1>)
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(mid_point_acceleration, 1.)
            .create();
        let _final_velocity = step
            .compute_velocity(DtFraction::<1, 1>, final_pos)
            .based_on(p0.v)
            .add_acceleration_dt(mid_point_acceleration, 1.)
            .create();
        step

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
        1
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

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        acceleration_field: &dyn AccelerationField,
    ) -> Step {
        let mut step = Step::new_deprecated(self.expected_capacities_for_step(), dt);
        let p0 = step.set_start_condition(current);
        let mid_point_pos = step
            .compute_position(DtFraction::<1, 2>)
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(p0.a, 0.5)
            .create();
        let mid_point_acceleration =
            step.compute_acceleration_at(mid_point_pos, acceleration_field);
        let final_pos = step
            .compute_position(DtFraction::<1, 1>)
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(mid_point_acceleration, 0.5)
            .create();
        let _final_velocity = step
            .compute_velocity(DtFraction::<1, 1>, final_pos)
            .based_on(p0.v)
            .add_acceleration_dt(mid_point_acceleration, 1.)
            .create();
        step

        // let p0 = current.tracker();

        // let dt_mid_point = fraction!(1 / 2) * dt;
        // let s_mid = p0.s + p0.v * dt_mid_point + 0.5 * p0.a * dt_mid_point * dt_mid_point;
        // let a_mid = s_mid.compute_acceleration(acceleration_field);

        // let v = p0.v + a_mid * dt;
        // let s = p0.s + p0.v * dt + 0.5 * a_mid * dt * dt;
        // s1 | v1;
    }

    fn expected_accelerations_for_step(&self) -> usize {
        2
    }

    fn expected_positions_for_step(&self) -> usize {
        2
    }

    fn expected_velocities_for_step(&self) -> usize {
        1
    }
}
