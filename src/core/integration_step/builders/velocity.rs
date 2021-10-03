use super::{
    core::{self, Fraction},
    integration_step::{
        contributions,
        step::{AccelerationRef, PositionRef, Step, VelocityRef},
    },
};

pub struct Velocity<'a> {
    step: &'a mut Step,
    dt_fraction: Fraction,
    s_ref: PositionRef,
    contributions: Vec<contributions::velocity::Variant>,
}

impl<'a> Velocity<'a> {
    pub fn new(step: &'a mut Step, dt_fraction: Fraction, s_ref: PositionRef) -> Self {
        Self {
            step,
            dt_fraction,
            s_ref,
            // most of the times there will be 2 contributions:
            contributions: Vec::with_capacity(2),
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
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> VelocityRef {
        let mut v = core::Velocity::zeros();
        for contrib in &self.contributions {
            v += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_velocity(v, self.s_ref, self.contributions)
    }
}
