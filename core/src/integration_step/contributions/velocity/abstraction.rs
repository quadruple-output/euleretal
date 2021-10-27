use super::{step::Step, Contribution, Variant};
use crate::{Fraction, PhysicalQuantityKind, Position, Vec3};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: Variant<Fraction>,
}

impl<'a> Abstraction<'a> {
    pub fn new(step: &'a Step, variant: Variant<Fraction>) -> Self {
        Self { step, variant }
    }
}

impl<'abstr> Contribution for Abstraction<'abstr> {
    fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::Velocity { v_ref, .. } => step[step[v_ref].sampling_position].s,
            Variant::AccelerationDt { a_ref, .. } => step[step[a_ref].sampling_position].s,
        }
    }

    fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    fn vector(&self) -> Option<Vec3> {
        Some(self.variant.evaluate_for(self.step).into())
    }

    fn contributions_factor(&self) -> f32 {
        match self.variant {
            Variant::Velocity { .. } => 1.,
            Variant::AccelerationDt {
                factor,
                dt_fraction,
                ..
            } => factor * dt_fraction,
        }
    }

    fn contributions_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a> {
        match self.variant {
            Variant::Velocity { v_ref } => {
                // todo: tidy up
                let velocity: &'a crate::integration_step::computed::Velocity = &self.step[v_ref];
                let abstraction: crate::integration_step::computed::velocity::Abstraction<'a> =
                    velocity.abstraction_for(self.step);
                let contributions = abstraction.contributions_iter();
                Box::new(contributions)
            }
            Variant::AccelerationDt { .. } => Box::new(::std::iter::empty()),
        }
    }
}
