use super::{
    core::{Acceleration, DtFraction, PhysicalQuantityKind, Position},
    step::{AccelerationRef, Step},
    velocity,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: &'a Variant,
}

// todo: this could be a trait, generic over the output type of vector()
impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::Acceleration { factor: _, a_ref } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        match self.variant {
            Variant::Acceleration { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub fn vector(&self) -> Acceleration {
        self.variant.evaluate_for(self.step)
    }
}

#[derive(Clone, Copy)]
pub enum Variant {
    Acceleration { factor: f32, a_ref: AccelerationRef },
}

impl From<AccelerationRef> for Variant {
    fn from(a_ref: AccelerationRef) -> Self {
        Self::Acceleration { factor: 1., a_ref }
    }
}

impl Variant {
    pub(in crate::core::integration_step) fn evaluate_for(&self, step: &Step) -> Acceleration {
        match *self {
            Self::Acceleration { factor, a_ref } => factor * step[a_ref].a,
        }
    }

    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            variant: self,
        }
    }
}

impl std::ops::Mul<DtFraction> for Variant {
    type Output = velocity::Variant;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self {
            Self::Acceleration { factor, a_ref } => velocity::Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            },
        }
    }
}

impl std::ops::Mul<Variant> for f32 {
    type Output = Variant;

    fn mul(self, rhs: Variant) -> Self::Output {
        match rhs {
            Variant::Acceleration { factor, a_ref } => Variant::Acceleration {
                factor: self * factor,
                a_ref,
            },
        }
    }
}
