use super::{
    core::{self, AccelerationField, StartCondition},
    integration_step::{
        contributions,
        step::{AccelerationRef, PositionRef, VelocityRef},
    },
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
    pub fn dt(&self) -> contributions::DtFraction {
        fraction!(1 / 1).into()
    }

    #[allow(clippy::unused_self)]
    pub fn start_values(
        &self,
    ) -> (
        contributions::position::Variant,
        contributions::velocity::Variant,
        contributions::acceleration::Variant,
    ) {
        (
            PositionRef::default().into(),
            VelocityRef::default().into(),
            AccelerationRef::default().into(),
        )
    }
}

pub trait Push<Contrib> {
    type Output;
    fn push(&mut self, _: Contrib) -> Self::Output;
}

impl<'a, IntoPosVariant> Push<IntoPosVariant> for Step<'a>
where
    IntoPosVariant: Into<contributions::position::Variant>,
{
    type Output = contributions::position::Variant;

    fn push(&mut self, position_contribution: IntoPosVariant) -> Self::Output {
        //todo: can be delegated to Push<contributions::position::Collection>
        let position_contribution = position_contribution.into();
        match position_contribution {
            contributions::position::Variant::StartPosition { s_ref } => {
                self.step.add_computed_position(
                    self.step[s_ref].s,
                    self.step[s_ref].dt_fraction,
                    vec![position_contribution].into(),
                )
            }
            contributions::position::Variant::VelocityDt {
                factor,
                v_ref,
                dt_fraction,
            } => todo!(),
            contributions::position::Variant::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            } => todo!(),
        }
        .into()
    }
}

impl<'a> Push<contributions::position::Collection> for Step<'a> {
    type Output = contributions::position::Variant;

    fn push(&mut self, contributions: contributions::position::Collection) -> Self::Output {
        let mut s = core::Position::origin();
        for contrib in &contributions {
            s += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_position(
                s,
                fraction!(1 / 1), //todo
                contributions,
            )
            .into()
    }
}

impl<'a> Push<contributions::velocity::Variant> for Step<'a> {
    type Output = contributions::velocity::Variant;

    fn push(&mut self, velocity_contribution: contributions::velocity::Variant) -> Self::Output {
        match velocity_contribution {
            contributions::velocity::Variant::Velocity { v_ref } => self
                .step
                .add_computed_velocity(
                    self.step[v_ref].v,
                    self.step[v_ref].sampling_position,
                    vec![velocity_contribution].into(),
                )
                .into(),
            contributions::velocity::Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            } => todo!(),
        }
    }
}

impl<'a> Push<contributions::velocity::Collection> for Step<'a> {
    type Output = contributions::velocity::Variant;

    fn push(&mut self, contributions: contributions::velocity::Collection) -> Self::Output {
        let mut v = core::Velocity::zeros();
        for contrib in &contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_velocity(
                v,
                self.step.last_position_ref(), // just a default
                contributions,
            )
            .into()
    }
}
