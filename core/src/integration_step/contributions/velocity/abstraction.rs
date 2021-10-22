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

impl<'a> Contribution<'a> for Abstraction<'a> {
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

    fn contributions_iter(&'a self) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a> {
        match self.variant {
            Variant::Velocity { v_ref } => Box::new(
                self.step[v_ref]
                    .abstraction_for(self.step)
                    .contributions_iter()
                    .map(|contrib| {
                        /*
                          Why does it have to be so complicated?

                          see https://stackoverflow.com/questions/52288980/how-does-the-mechanism-behind-the-creation-of-boxed-traits-work

                          and note:
                          "Coercions are only applied in coercion site like the return value. [or
                          else] no unsized coercion is performed by the compiler."
                          [https://stackoverflow.com/questions/65916882/cant-box-a-struct-that-implements-a-trait-as-a-trait-object]
                        */
                        let b: Box<dyn Contribution + 'a> = Box::new(contrib);
                        b
                    }),
            ),
            Variant::AccelerationDt { .. } => Box::new(::std::iter::empty()),
        }
    }
}
