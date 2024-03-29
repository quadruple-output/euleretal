use super::{step::Step, Contribution, Variant};
use crate::{Fraction, PhysicalQuantityKind, Position, Vec3};

#[derive(Clone)]
pub struct Abstraction<'a> {
    step: &'a Step,
    // Abstraction cannot be parameterized, so we move the static fraction to a component
    variant: Variant<Fraction>,
}

impl<'a> Abstraction<'a> {
    pub fn new(step: &'a Step, variant: Variant<Fraction>) -> Self {
        Self { step, variant }
    }
}

impl<'abst> Contribution for Abstraction<'abst> {
    fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::StartPosition { s_ref } => step[s_ref].s,
            Variant::VelocityDt { v_ref, .. } => step[step[v_ref].sampling_position].s,
            Variant::AccelerationDtDt { a_ref, .. } => step[step[a_ref].sampling_position].s,
        }
    }

    fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    fn vector(&self) -> Option<Vec3> {
        match self.variant {
            Variant::StartPosition { .. } => None,
            Variant::VelocityDt { .. } | Variant::AccelerationDtDt { .. } => {
                Some(self.variant.evaluate_for(self.step).into())
            }
        }
    }

    fn contributions_factor(&self) -> f32 {
        match self.variant {
            Variant::StartPosition { .. } => 1.,
            Variant::VelocityDt {
                factor,
                dt_fraction,
                ..
            }
            | Variant::AccelerationDtDt {
                factor,
                dt_fraction,
                ..
            } => factor * dt_fraction,
        }
    }

    fn contributions_iter(&self) -> Box<dyn Iterator<Item = Box<dyn Contribution + '_>> + '_> {
        match self.variant {
            Variant::StartPosition { s_ref } => {
                let iter = self.step[s_ref]
                    .abstraction_for(self.step)
                    .contributions_iter()
                    .map(|contribution| {
                        Box::new(contribution) as Box<dyn Contribution>
                        /*
                          Why do we need the cast here, but not below?
                          ⟶ https://stackoverflow.com/questions/52288980/how-does-the-mechanism-behind-the-creation-of-boxed-traits-work
                          and note: “Coercions are only applied in coercion site like the return
                          value. [or else] no unsized coercion is performed by the compiler.”
                          [https://stackoverflow.com/questions/65916882/cant-box-a-struct-that-implements-a-trait-as-a-trait-object]
                        */
                    });
                Box::new(iter)
            }
            Variant::VelocityDt { v_ref, .. } => Box::new(
                self.step[v_ref]
                    .abstraction_for(self.step)
                    .contributions_iter(),
            ),
            Variant::AccelerationDtDt { .. } => {
                // accelerations do not have any contributions
                Box::new(::std::iter::empty())
            }
        }
    }
}
