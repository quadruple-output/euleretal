use super::core::{AccelerationField, DtFraction, Duration, Integrator, StartCondition, Step};

pub struct ExactForConst {}

impl ExactForConst {
    pub fn new() -> Self {
        ExactForConst {}
    }
}

impl Integrator for ExactForConst {
    fn label(&self) -> String {
        "Exact for const. acceleration".to_string()
    }

    fn description(&self) -> String {
        "v' = v + a dt\n\
         s' = s + v dt + ½ a dt²"
            .to_string()
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        dt: Duration,
        _acceleration_field: &dyn AccelerationField,
    ) -> Step {
        // let p0 = current.tracker();
        // let v1 = p0.v + p0.a * dt; // dt: Duration
        // let s1 = p0.s + p0.v * dt + 0.5 * p0.a * dt * dt;
        // s1 | v1;

        let mut step = Step::new_deprecated(self.expected_capacities_for_step(), dt);
        let p0 = step.set_start_condition(current);
        step.compute_velocity(DtFraction::<1, 1>, p0.s)
            .based_on(p0.v)
            .add_acceleration_dt(p0.a, 1.)
            .create();
        step.compute_position(DtFraction::<1, 1>)
            .based_on(p0.s)
            .add_velocity_dt(p0.v, 1.)
            .add_acceleration_dt_dt(p0.a, 0.5)
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
