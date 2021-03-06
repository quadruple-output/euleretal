use super::core::{AccelerationField, Duration, IntegrationStep, Integrator, StartCondition};

pub struct Broken {}

impl Broken {
    pub fn new() -> Self {
        Broken {}
    }
}

impl Integrator for Broken {
    fn label(&self) -> String {
        "Broken Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v dt"
            .to_string()
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        _acceleration_field: &dyn AccelerationField,
    ) -> IntegrationStep {
        let mut step = IntegrationStep::new(self.expected_capacities_for_step(), dt);
        let p0 = step.initial_condition(current);
        step.compute_position(fraction!(1 / 1))
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            //.add_acceleration_dt_dt(p0.a, 1.)
            .create();
        step.compute_velocity(fraction!(1 / 1), p0.s)
            .based_on(p0.v)
            .add_acceleration_dt(p0.a, 1.)
            .create();
        step
    }

    fn expected_accelerations_for_step(&self) -> usize {
        1
    }

    fn expected_positions_for_step(&self) -> usize {
        1
    }

    fn expected_velocities_for_step(&self) -> usize {
        1
    }
}

pub struct Euler {}

impl Euler {
    pub fn new() -> Self {
        Euler {}
    }
}

impl Integrator for Euler {
    fn label(&self) -> String {
        "Euler".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v' dt\n\
             = s + v dt + a dt²" // !! this string contains non-breaking spaces
            .to_string()
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        _acceleration_field: &dyn AccelerationField,
    ) -> IntegrationStep {
        let mut step = IntegrationStep::new(self.expected_capacities_for_step(), dt);
        let p0 = step.initial_condition(current);
        let next_position = step
            .compute_position(fraction!(1 / 1))
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(p0.a, 1.)
            .create();
        let _next_velocity = step
            .compute_velocity(fraction!(1 / 1), next_position)
            .based_on(p0.v)
            .add_acceleration_dt(p0.a, 1.)
            .create();
        step
    }

    fn expected_accelerations_for_step(&self) -> usize {
        1
    }

    fn expected_positions_for_step(&self) -> usize {
        1
    }

    fn expected_velocities_for_step(&self) -> usize {
        1
    }
}
