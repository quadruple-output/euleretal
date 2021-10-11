use super::{
    core::{self, DtFraction},
    integration_step::{
        contributions,
        step::{AccelerationRef, PositionRef, Step, VelocityRef},
    },
};

pub struct VelocityDeprecated<'a, const N: usize, const D: usize> {
    step: &'a mut Step,
    s_ref: PositionRef,
    contributions: contributions::velocity::Collection<N, D>,
}

impl<'a, const N: usize, const D: usize> VelocityDeprecated<'a, N, D> {
    pub fn new(step: &'a mut Step, s_ref: PositionRef) -> Self {
        Self {
            step,
            s_ref,
            // most of the times there will be 2 contributions:
            contributions: contributions::velocity::Collection::with_capacity(2),
        }
    }

    pub fn based_on(mut self, v_ref: VelocityRef) -> Self {
        self.contributions
            .push(contributions::velocity::Variant::Velocity { v_ref });
        self
    }

    pub fn add_acceleration_dt(mut self, a_ref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(contributions::velocity::Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction: DtFraction,
            });
        self
    }

    pub fn create(self) -> VelocityRef {
        let mut v = core::Velocity::zeros();
        for contrib in &self.contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_velocity(v, self.s_ref, DtFraction::<N, D>, self.contributions)
    }
}
