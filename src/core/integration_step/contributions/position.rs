use super::{
    core::{Move, PhysicalQuantityKind, Position},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
    DtFraction,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    // Abstraction cannot be parameterized, so we move the static fraction to a component
    variant: Variant<1, 1>,
    variant_scale: f32,
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::StartPosition { s_ref } => step[s_ref].s,
            Variant::VelocityDt { v_ref, .. } => step[step[v_ref].sampling_position].s,
            Variant::AccelerationDtDt { a_ref, .. } => step[step[a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    pub fn vector(&self) -> Option<Move> {
        match self.variant {
            Variant::StartPosition { .. } => None,
            Variant::VelocityDt { .. } => {
                Some(self.variant.evaluate_for(self.step) * self.variant_scale)
            }
            Variant::AccelerationDtDt { .. } => {
                Some(self.variant.evaluate_for(self.step) * self.variant_scale * self.variant_scale)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Variant<const N: usize, const D: usize> {
    StartPosition {
        s_ref: PositionRef,
    },
    VelocityDt {
        factor: f32,
        v_ref: VelocityRef,
        dt_fraction: DtFraction<N, D>,
    },
    AccelerationDtDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: DtFraction<N, D>,
    },
}

impl<const N: usize, const D: usize> From<PositionRef> for Variant<N, D> {
    fn from(s_ref: PositionRef) -> Self {
        Self::StartPosition { s_ref }
    }
}

impl<const N: usize, const D: usize> Variant<N, D> {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::StartPosition { .. } => PhysicalQuantityKind::Position,
            Self::VelocityDt { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(in super::super) fn evaluate_for(&self, step: &Step) -> Move {
        match *self {
            Self::StartPosition { s_ref } => step[s_ref].s.into(),
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

    pub(in super::super) fn abstraction_scaled_for<'a>(
        &'a self,
        step: &'a Step,
        scale: f32,
    ) -> Abstraction<'a> {
        Abstraction {
            step,
            variant: self.transmute(),
            variant_scale: scale,
        }
    }

    fn transmute<const A: usize, const B: usize>(self) -> Variant<A, B> {
        unsafe { ::std::mem::transmute::<Self, Variant<A, B>>(self) }
    }
}

impl<const N: usize, const D: usize> std::ops::Add for Variant<N, D> {
    type Output = Collection<N, D>;

    fn add(self, rhs: Self) -> Self::Output {
        vec![self, rhs].into()
    }
}

#[derive(Default)]
pub struct Collection<const N: usize, const D: usize>(pub(in super::super) Vec<Variant<N, D>>);

impl<const N: usize, const D: usize> From<Vec<Variant<N, D>>> for Collection<N, D> {
    fn from(vec: Vec<Variant<N, D>>) -> Self {
        Self(vec)
    }
}

impl<const N: usize, const D: usize> Collection<N, D> {
    pub(in super::super) const fn empty() -> Self {
        Self(Vec::new())
    }

    pub(in super::super) fn iter(&self) -> impl Iterator<Item = &Variant<N, D>> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn transmute<const A: usize, const B: usize>(self) -> Collection<A, B> {
        unsafe { ::std::mem::transmute::<Self, Collection<A, B>>(self) }
    }
}

impl<'a, const N: usize, const D: usize> IntoIterator for &'a Collection<N, D> {
    type Item = &'a Variant<N, D>;

    type IntoIter = std::slice::Iter<'a, Variant<N, D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<const N: usize, const D: usize> std::ops::Add<Variant<N, D>> for Collection<N, D> {
    type Output = Self;

    fn add(self, rhs: Variant<N, D>) -> Self::Output {
        Self(self.0.into_iter().chain(Some(rhs)).collect())
    }
}
