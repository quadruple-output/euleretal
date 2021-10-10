use super::{
    core::{self, AccelerationField, DtFraction, StartCondition},
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
    pub fn dt(&self) -> DtFraction {
        fraction!(1 / 1).into()
    }

    #[allow(clippy::unused_self)]
    pub fn start_values(&self) -> (PositionRef, VelocityRef, AccelerationRef) {
        (
            PositionRef::default(),
            VelocityRef::default(),
            AccelerationRef::default(),
        )
    }

    pub fn set_display_position(&mut self, v_ref: VelocityRef, s_ref: PositionRef) {
        self.step[v_ref].sampling_position = s_ref;
    }
}

pub trait Collector<Contribution> {
    type Output;
    fn push(&mut self, _: Contribution) -> Self::Output;
}

impl<'a> Collector<PositionRef> for Step<'a> {
    type Output = PositionRef;

    fn push(&mut self, s_ref: PositionRef) -> Self::Output {
        self.step.add_computed_position(
            self.step[s_ref].s,
            self.step[s_ref].dt_fraction,
            vec![contributions::position::Variant::StartPosition { s_ref }].into(),
        )
    }
}

impl<'a> Collector<contributions::position::Collection> for Step<'a> {
    type Output = PositionRef;

    fn push(&mut self, contributions: contributions::position::Collection) -> Self::Output {
        let mut s = core::Position::origin();
        for contrib in &contributions {
            s += contrib.evaluate_for(self.step);
        }
        self.step.add_computed_position(
            s,
            *&(fraction!(1 / 1).into()), //todo
            contributions,
        )
    }
}

impl<'a> Collector<VelocityRef> for Step<'a> {
    type Output = VelocityRef;

    fn push(&mut self, v_ref: VelocityRef) -> Self::Output {
        self.step.add_computed_velocity(
            self.step[v_ref].v,
            self.step[v_ref].sampling_position,
            vec![contributions::velocity::Variant::Velocity { v_ref }].into(),
        )
    }
}

impl<'a> Collector<contributions::velocity::Collection> for Step<'a> {
    type Output = VelocityRef;

    fn push(&mut self, contributions: contributions::velocity::Collection) -> Self::Output {
        let mut v = core::Velocity::zeros();
        for contrib in &contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step.add_computed_velocity(
            v,
            self.step.last_position_ref(), // just a default
            contributions,
        )
    }
}
