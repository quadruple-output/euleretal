use super::{
    core::{Fraction, Move, PhysicalQuantityKind, Position},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    data: &'a Variant,
}

pub(in crate::core::integration_step) enum Variant {
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

pub struct Collection(pub(in crate::core::integration_step) Vec<Variant>);

impl From<Vec<Variant>> for Collection {
    fn from(v: Vec<Variant>) -> Self {
        Self(v)
    }
}

impl Collection {
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(in crate::core::integration_step) fn iter(&self) -> impl Iterator<Item = &Variant> {
        self.0.iter()
    }

    pub(in crate::core::integration_step) fn push(&mut self, data: Variant) {
        self.0.push(data);
    }
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            Variant::StartPosition { s_ref } => step[*s_ref].s,
            Variant::VelocityDt { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Variant::AccelerationDtDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Option<Move> {
        match self.data {
            Variant::StartPosition { .. } => None,
            _ => Some(self.data.evaluate_for(self.step)),
        }
    }
}

impl Variant {
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

    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        Abstraction { step, data: self }
    }
}
