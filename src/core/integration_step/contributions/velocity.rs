use super::{
    core::{Fraction, PhysicalQuantityKind, Position, Velocity},
    step::{AccelerationRef, Step, VelocityRef},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    data: &'a Variant,
}

pub(in crate::core::integration_step) enum Variant {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Variant::Velocity { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Variant::AccelerationDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Velocity {
        self.data.evaluate_for(self.step)
    }
}

impl Variant {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::Velocity { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(in crate::core::integration_step) fn evaluate_for(&self, step: &Step) -> Velocity {
        match *self {
            Self::Velocity { v_ref: vref } => step[vref].v,
            Self::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            } => factor * &step[a_ref] * dt_fraction * step.dt(),
        }
    }

    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction { step, data: self }
    }
}
