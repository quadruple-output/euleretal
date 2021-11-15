use crate::{
    integration_step::builders, Acceleration, AccelerationField, Duration, Integrator, Position,
    StartCondition, Step, Velocity,
};

#[derive(Clone, Copy, ::serde::Deserialize, ::serde::Serialize)]
pub struct CenterMass;

impl AccelerationField for CenterMass {
    fn value_at(&self, pos: Position) -> Acceleration {
        let distance_squared_recip = pos.as_vector().norm_squared().recip();
        (-pos.as_vector() * distance_squared_recip.sqrt() * distance_squared_recip).into()
    }

    fn label(&self) -> String {
        "Gravity".to_string()
    }
    fn to_concrete_type(
        &self,
    ) -> crate::scenarios::serde_box_dyn_acceleration_field::AccelerationFieldSerDe {
        unimplemented!() // not required for test helpers
    }
}

pub struct TestSetup {
    acceleration_field: CenterMass,
    start_condition: StartCondition,
    dt: Duration,
}

impl Default for TestSetup {
    fn default() -> Self {
        let acceleration_field = CenterMass;
        let start_position = Position::new(1., 2., 3.);
        Self {
            acceleration_field,
            start_condition: StartCondition::new(
                start_position,
                Velocity::new(4., 5., 6.),
                acceleration_field.value_at(start_position),
            ),
            dt: 0.3.into(),
        }
    }
}

impl TestSetup {
    fn new_builder_for<'a>(&'a self, step: &'a mut Step) -> builders::Step<'a> {
        builders::Step::new(&self.acceleration_field, step)
    }

    fn new_step(&self) -> Step {
        Step::new(&self.start_condition, self.dt)
    }

    pub fn assert_first_step(
        &self,
        integrator: &dyn Integrator,
        expected: impl Fn(
            Position,
            Velocity,
            Acceleration,
            &dyn AccelerationField,
            Duration,
        ) -> (Position, Velocity),
    ) {
        let mut step = self.new_step();
        let mut builder = self.new_builder_for(&mut step);
        let ((s, v, a), dt) = (builder.start_values(), builder.dt());
        integrator.integrate_step(s, v, a, dt, &mut builder);
        builder.finalize();
        let (s, v, a, dt) = (
            self.start_condition.position(),
            self.start_condition.velocity(),
            self.start_condition.acceleration(),
            self.dt,
        );
        let (exp_s, exp_v) = expected(s, v, a, &self.acceleration_field, dt);
        assert_eq!(step.last_s(), exp_s);
        assert_eq!(step.last_v(), exp_v);
    }
}
