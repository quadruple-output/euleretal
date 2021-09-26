use super::{
    acceleration::Acceleration1,
    core::{self, integrator::ExpectedCapacities, AccelerationField, Duration, StartCondition},
    integration_step::{
        step::{PositionRef, VelocityRef},
        PositionContributionData, VelocityContributionData,
    },
    position::Position1,
    velocity::Velocity1,
};

pub struct Step<'a> {
    acceleration_field: &'a dyn AccelerationField,
    start_condition: StartCondition,
    step: core::Step,
}

pub trait Push<T> {
    fn push(&mut self, _: T);
}

impl<'a> Step<'a> {
    pub fn new(
        acceleration_field: &'a dyn AccelerationField,
        start_condition: &StartCondition,
        dt: Duration,
    ) -> Self {
        let mut step = core::Step::new(ExpectedCapacities::default(), dt);
        step.set_start_condition(start_condition);
        Self {
            acceleration_field,
            start_condition: start_condition.clone(),
            step,
        }
    }

    pub fn from_previous(
        acceleration_field: &'a dyn AccelerationField,
        previous: &core::Step,
    ) -> Self {
        Self {
            acceleration_field,
            start_condition: previous.get_start_condition(),
            step: core::Step::from_previous(previous),
        }
    }

    pub fn result(mut self) -> core::Step {
        self.step
            .compute_acceleration_at_last_position(self.acceleration_field);
        self.step
    }

    pub fn dt(&self) -> Duration {
        self.step.dt()
    }

    pub fn start_values(&self) -> (Position1, Velocity1, Acceleration1) {
        (
            PositionRef::default().into(),
            VelocityRef::default().into(),
            //AccelerationRef::default().into(),
            self.start_condition.acceleration().into(),
        )
    }
}

impl<'a> Push<Position1> for Step<'a> {
    fn push(&mut self, p: Position1) {
        let s_ref: PositionRef = p.into();
        self.step.add_computed_position(
            self.step[s_ref].s,
            self.step[s_ref].dt_fraction,
            vec![PositionContributionData::StartPosition { s_ref }],
        );
    }
}

impl<'a> Push<Velocity1> for Step<'a> {
    fn push(&mut self, v: Velocity1) {
        let v_ref: VelocityRef = v.into();
        self.step.add_computed_velocity(
            self.step[v_ref].v,
            self.step[v_ref].sampling_position,
            vec![VelocityContributionData::Velocity { v_ref }],
        );
    }
}
