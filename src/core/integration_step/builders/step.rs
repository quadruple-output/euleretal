use super::{
    acceleration::Acceleration1,
    core::{self, integrator::ExpectedCapacities, Duration, StartCondition},
    integration_step::{step::PositionRef, PositionContributionData},
    position::Position1,
    velocity::Velocity1,
};

pub struct Step {
    dt: Duration,
    start_condition: StartCondition,
    step: core::Step,
}

pub trait Push<T> {
    fn push(&mut self, _: T);
}

impl Step {
    pub fn new(start_condition: &StartCondition, dt: Duration) -> Self {
        let mut step = core::Step::new(ExpectedCapacities::default(), dt);
        let start_pos_ref = step.start_position(start_condition.position());
        step.start_velocity(start_condition.velocity(), start_pos_ref);
        step.start_acceleration(start_condition.acceleration(), start_pos_ref);
        Self {
            dt,
            start_condition: start_condition.clone(),
            step,
        }
    }

    pub fn result(self) -> core::Step {
        self.step
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn start_values(&self) -> (Position1, Velocity1, Acceleration1) {
        (
            PositionRef::default().into(),
            //VelocityRef::default().into(),
            //AccelerationRef::default().into(),
            self.start_condition.velocity().into(),
            self.start_condition.acceleration().into(),
        )
    }
}

impl Push<Position1> for Step {
    fn push(&mut self, p: Position1) {
        let s_ref: PositionRef = p.into();
        let s = self.step[s_ref].s;
        self.step.add_computed_position(
            s,
            fraction!(0 / 1),
            vec![PositionContributionData::StartPosition { s_ref }],
        );
    }
}
