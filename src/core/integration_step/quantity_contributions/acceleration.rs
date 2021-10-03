use super::{
    core::{Acceleration, PhysicalQuantityKind, Position},
    step::{AccelerationRef, Step},
};

pub struct Contribution<'a> {
    step: &'a Step,
    data: &'a Data,
}

#[derive(Clone, Copy)]
pub(in crate::core::integration_step) enum Data {
    Acceleration { factor: f32, a_ref: AccelerationRef },
}

// todo: this could be a trait, generic over the output type of vector()
impl<'a> Contribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Data::Acceleration { factor: _, a_ref } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Acceleration {
        self.data.evaluate_for(self.step)
    }
}

impl Data {
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
    ) -> Contribution<'a> {
        Contribution { step, data: self }
    }
}
