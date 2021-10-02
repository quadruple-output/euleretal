use super::{
    acceleration::Acceleration1,
    core::{self, integrator::ExpectedCapacities, AccelerationField, Duration, StartCondition},
    integration_step::{
        step::{PositionRef, VelocityRef},
        PositionContributionData, PositionContributionDataCollection, VelocityContributionData,
    },
    position::PositionContribution,
    velocity::VelocityContribution,
    DtFraction,
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

    pub fn dt(&self) -> DtFraction {
        fraction!(1 / 1).into()
    }

    pub fn start_values(&self) -> (PositionContribution, VelocityContribution, Acceleration1) {
        (
            PositionRef::default().into(),
            VelocityRef::default().into(),
            //AccelerationRef::default().into(),
            self.start_condition.acceleration().into(),
        )
    }
}

impl<'a> Push<PositionContribution> for Step<'a> {
    fn push(&mut self, p: PositionContribution) {
        let position_contribution = PositionContributionData::from(p);
        match position_contribution {
            PositionContributionData::StartPosition { s_ref } => {
                self.step.add_computed_position(
                    self.step[s_ref].s,
                    self.step[s_ref].dt_fraction,
                    PositionContributionDataCollection(vec![position_contribution]),
                );
            }
            PositionContributionData::VelocityDt {
                factor,
                v_ref,
                dt_fraction,
            } => todo!(),
            PositionContributionData::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            } => todo!(),
        };
    }
}

impl<'a> Push<VelocityContribution> for Step<'a> {
    fn push(&mut self, v: VelocityContribution) {
        let velocity_contribution = VelocityContributionData::from(v);
        match velocity_contribution {
            VelocityContributionData::Velocity { v_ref } => {
                self.step.add_computed_velocity(
                    self.step[v_ref].v,
                    self.step[v_ref].sampling_position,
                    vec![velocity_contribution],
                );
            }
            VelocityContributionData::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            } => todo!(),
        }
    }
}

impl<'a> Push<PositionContributionDataCollection> for Step<'a> {
    fn push(&mut self, contributions: PositionContributionDataCollection) {
        let mut s = core::Position::origin();
        for contrib in contributions.iter() {
            s += contrib.evaluate_for(&self.step);
        }
        self.step.add_computed_position(
            s,
            fraction!(1 / 1), //todo
            contributions,
        );
    }
}
