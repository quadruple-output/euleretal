use super::{
    core::{Acceleration, PhysicalQuantityKind, Position},
    step::{AccelerationRef, Step},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    data: &'a Variant,
}

#[derive(Clone, Copy)]
pub(in crate::core::integration_step) enum Variant {
    Acceleration { factor: f32, a_ref: AccelerationRef },
}

// todo: this could be a trait, generic over the output type of vector()
impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Variant::Acceleration { factor: _, a_ref } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Acceleration {
        self.data.evaluate_for(self.step)
    }
}

impl Variant {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::Acceleration { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(in crate::core::integration_step) fn evaluate_for(&self, step: &Step) -> Acceleration {
        match *self {
            Self::Acceleration { factor, a_ref } => factor * step[a_ref].a,
        }
    }

    pub(in crate::core::integration_step) fn public_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction { step, data: self }
    }
}
