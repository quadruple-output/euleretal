use super::{
    core::{Fraction, PhysicalQuantityKind, Position, Velocity},
    step::{AccelerationRef, Step, VelocityRef},
};

pub struct Contribution<'a> {
    step: &'a Step,
    data: &'a Data,
}

pub(in crate::core::integration_step) enum Data {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

impl<'a> Contribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Data::Velocity { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Data::AccelerationDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Velocity {
        self.data.evaluate_for(self.step)
    }
}

impl Data {
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

    pub(in crate::core::integration_step) fn public_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Contribution<'a> {
        Contribution { step, data: self }
    }
}
