use super::{
    core::{DtFraction, Move, PhysicalQuantityKind, Position},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: &'a Variant,
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::StartPosition { s_ref } => step[*s_ref].s,
            Variant::VelocityDt { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Variant::AccelerationDtDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    pub fn vector(&self) -> Option<Move> {
        match self.variant {
            Variant::StartPosition { .. } => None,
            _ => Some(self.variant.evaluate_for(self.step)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    StartPosition {
        s_ref: PositionRef,
    },
    VelocityDt {
        factor: f32,
        v_ref: VelocityRef,
        dt_fraction: DtFraction,
    },
    AccelerationDtDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: DtFraction,
    },
}

impl From<PositionRef> for Variant {
    fn from(s_ref: PositionRef) -> Self {
        Self::StartPosition { s_ref }
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
            } => {
                let dt = dt_fraction * step.dt();
                let v = step[v_ref].v;
                factor * v * dt
            }
            Self::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            } => {
                let dt = dt_fraction * step.dt();
                let a = step[a_ref].a;
                factor * a * dt * dt
            }
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

impl std::ops::Add for Variant {
    type Output = Collection;

    fn add(self, rhs: Self) -> Self::Output {
        vec![self, rhs].into()
    }
}

#[derive(Default)]
pub struct Collection(pub(in crate::core::integration_step) Vec<Variant>);

impl From<Vec<Variant>> for Collection {
    fn from(v: Vec<Variant>) -> Self {
        Self(v)
    }
}

impl Collection {
    pub(in crate::core::integration_step) const fn empty() -> Self {
        Self(Vec::new())
    }

    pub(in crate::core::integration_step) fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(in crate::core::integration_step) fn iter(&self) -> impl Iterator<Item = &Variant> {
        self.0.iter()
    }

    pub(in crate::core::integration_step) fn push(&mut self, data: Variant) {
        self.0.push(data);
    }
}

impl<'a> IntoIterator for &'a Collection {
    type Item = &'a Variant;

    type IntoIter = std::slice::Iter<'a, Variant>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl std::ops::Add<Variant> for Collection {
    type Output = Self;

    fn add(self, rhs: Variant) -> Self::Output {
        Self(self.0.into_iter().chain(Some(rhs)).collect())
    }
}
