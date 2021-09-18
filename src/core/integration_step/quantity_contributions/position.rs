use super::{
    core::{Fraction, Move, PhysicalQuantityKind, Position},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
};

pub struct Contribution<'a> {
    step: &'a Step,
    data: &'a Data,
}

pub(in crate::core::integration_step) enum Data {
    StartPosition {
        s_ref: PositionRef,
    },
    VelocityDt {
        factor: f32,
        v_ref: VelocityRef,
        dt_fraction: Fraction,
    },
    AccelerationDtDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

impl<'a> Contribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Data::StartPosition { s_ref } => step[*s_ref].s,
            Data::VelocityDt { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Data::AccelerationDtDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Option<Move> {
        match self.data {
            Data::StartPosition { .. } => None,
            _ => Some(self.data.evaluate_for(self.step)),
        }
    }
}

impl Data {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::StartPosition { .. } => PhysicalQuantityKind::Position,
            Self::VelocityDt { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(in crate::core::integration_step) fn evaluate_for(&self, step: &Step) -> Move {
        match *self {
            Self::StartPosition { s_ref: sref } => step[sref].s.into(),
            Self::VelocityDt {
                factor,
                v_ref,
                dt_fraction,
            } => factor * &step[v_ref] * dt_fraction * step.dt(),
            Self::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            } => factor * &step[a_ref] * (dt_fraction * step.dt()) * (dt_fraction * step.dt()),
        }
    }

    pub(in crate::core::integration_step) fn public_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Contribution<'a> {
        Contribution { step, data: self }
    }
}
