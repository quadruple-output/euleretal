use super::{
    acceleration::AccelerationContribution,
    core::{self, AccelerationField, Duration, StartCondition},
    integration_step::{
        step::{AccelerationRef, PositionRef, VelocityRef},
        PositionContributionData, PositionContributionDataCollection, VelocityContributionData,
    },
    position::PositionContribution,
    velocity::VelocityContribution,
    DtFraction,
};

pub struct Step<'a> {
    acceleration_field: &'a dyn AccelerationField,
    step: &'a mut core::Step,
    #[cfg(debug_assertions)]
    finalized: bool,
}

impl<'a> Step<'a> {
    pub fn new<'b>(
        acceleration_field: &'a dyn AccelerationField,
        start_condition: &'b StartCondition,
        dt: Duration,
        step: &'a mut core::Step,
    ) -> Self {
        step.set_start_condition(start_condition);
        Self {
            acceleration_field,
            step,
            #[cfg(debug_assertions)]
            finalized: false,
        }
    }

    pub fn finalize(&mut self) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.finalized);
            self.finalized = true;
        }
        self.step
            .compute_acceleration_at_last_position(self.acceleration_field);
    }

    pub fn next_step(&self) -> core::Step {
        self.step.new_next()
    }

    pub fn next_for(self, step: &'a mut core::Step) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(self.finalized);
        Self {
            acceleration_field: self.acceleration_field,
            step,
            #[cfg(debug_assertions)]
            finalized: false,
        }
    }

    #[allow(clippy::unused_self)]
    pub fn dt(&self) -> DtFraction {
        fraction!(1 / 1).into()
    }

    #[allow(clippy::unused_self)]
    pub fn start_values(
        &self,
    ) -> (
        PositionContribution,
        VelocityContribution,
        AccelerationContribution,
    ) {
        (
            PositionRef::default().into(),
            VelocityRef::default().into(),
            AccelerationRef::default().into(),
        )
    }
}

pub trait Push<T> {
    fn push(&mut self, _: T);
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
            s += contrib.evaluate_for(self.step);
        }
        self.step.add_computed_position(
            s,
            fraction!(1 / 1), //todo
            contributions,
        );
    }
}
