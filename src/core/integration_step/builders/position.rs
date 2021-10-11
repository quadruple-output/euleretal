use super::{
    super::integration_step::{
        contributions,
        step::{AccelerationRef, PositionRef, VelocityRef},
        Step,
    },
    core::{self, DtFraction},
};

pub struct PositionDeprecated<'a, const N: usize, const D: usize> {
    step: &'a mut Step,
    contributions: contributions::position::Collection<N, D>,
}

impl<'a, const N: usize, const D: usize> PositionDeprecated<'a, N, D> {
    pub fn new(step: &'a mut Step) -> Self {
        Self {
            step,
            // most of the times there will be 3 contributions:
            contributions: contributions::position::Collection::with_capacity(3),
        }
    }

    pub fn based_on(mut self, s_ref: PositionRef) -> Self {
        self.contributions
            .push(contributions::position::Variant::StartPosition { s_ref });
        self
    }

    pub fn add_velocity_dt(mut self, v_ref: VelocityRef, factor: f32) -> Self {
        self.contributions
            .push(contributions::position::Variant::VelocityDt {
                factor,
                v_ref,
                dt_fraction: DtFraction,
            });
        self
    }

    pub fn add_acceleration_dt_dt(mut self, a_ref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(contributions::position::Variant::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction: DtFraction,
            });
        self
    }

    pub fn create(self) -> PositionRef {
        let mut s = core::Position::origin();
        for contrib in &self.contributions {
            s += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_position(s, DtFraction::<N, D>, self.contributions)
    }
}
