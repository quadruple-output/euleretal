use super::{
    contributions,
    core::{self, Position},
    step::{PositionRef, Step},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    velocity: &'a Velocity,
}

/// This type must be public because it is returned by the impl of
/// [`::std::ops::Index`] for [`IntegrationStep`]. All members are non-public,
/// however, such that it cannot be used from outside.
pub struct Velocity {
    pub(in crate::core::integration_step) v: core::Velocity,
    pub(in crate::core::integration_step) sampling_position: PositionRef,
    pub(in crate::core::integration_step) contributions: Vec<contributions::velocity::Variant>,
}

impl<'a> Abstraction<'a> {
    pub fn v(&self) -> core::Velocity {
        self.velocity.v
    }

    pub fn sampling_position(&self) -> Position {
        self.step[self.velocity.sampling_position].s
    }

    pub fn contributions_iter(
        &'a self,
    ) -> impl Iterator<Item = contributions::velocity::Abstraction<'a>> {
        self.velocity
            .contributions
            .iter()
            .map(move |contrib| contrib.abstraction_for(self.step))
    }
}

impl ::std::ops::Mul<&Velocity> for f32 {
    type Output = core::Velocity;

    fn mul(self, rhs: &Velocity) -> Self::Output {
        self * rhs.v
    }
}

impl Velocity {
    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            velocity: self,
        }
    }
}
