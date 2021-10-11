use super::{
    core::{DtFraction, PhysicalQuantityKind, Position, Velocity},
    position,
    step::{AccelerationRef, Step, VelocityRef},
};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: Variant<1, 1>,
    variant_scale: f32,
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::Velocity { v_ref, .. } => step[step[v_ref].sampling_position].s,
            Variant::AccelerationDt { a_ref, .. } => step[step[a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    pub fn vector(&self) -> Velocity {
        match self.variant {
            Variant::Velocity { .. } => self.variant.evaluate_for(self.step),
            Variant::AccelerationDt { .. } => {
                self.variant.evaluate_for(self.step) * self.variant_scale
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Variant<const N: usize, const D: usize> {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: DtFraction<N, D>,
    },
}

impl<const N: usize, const D: usize> From<VelocityRef> for Variant<N, D> {
    fn from(v_ref: VelocityRef) -> Self {
        Self::Velocity { v_ref }
    }
}

impl<const N: usize, const D: usize> Variant<N, D> {
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
            } => {
                let dt = dt_fraction * step.dt();
                let a = step[a_ref].a;
                factor * a * dt
            }
        }
    }

    pub(in crate::core::integration_step) fn abstraction_scaled_for<'a>(
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

    pub(in crate::core::integration_step) fn abstraction_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> Abstraction<'a> {
        self.abstraction_scaled_for(step, DtFraction::<N, D>.into())
    }

    fn transmute<const A: usize, const B: usize>(self) -> Variant<A, B> {
        unsafe { ::std::mem::transmute::<Self, Variant<A, B>>(self) }
    }
}

impl<const N: usize, const D: usize> std::ops::Add for Variant<N, D> {
    type Output = Collection<N, D>;

    fn add(self, rhs: Variant<N, D>) -> Self::Output {
        vec![self, rhs].into()
    }
}

impl<const N: usize, const D: usize> std::ops::Mul<DtFraction<N, D>> for Variant<N, D> {
    type Output = position::Variant<N, D>;

    fn mul(self, dt_fraction: DtFraction<N, D>) -> Self::Output {
        match self {
            Variant::Velocity { v_ref } => position::Variant::VelocityDt {
                factor: 1.,
                v_ref,
                dt_fraction,
            },
            Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            } => position::Variant::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            },
        }
    }
}

#[derive(Default)]
pub struct Collection<const N: usize, const D: usize>(
    pub(in crate::core::integration_step) Vec<Variant<N, D>>,
);

impl<const N: usize, const D: usize> From<Vec<Variant<N, D>>> for Collection<N, D> {
    fn from(vec: Vec<Variant<N, D>>) -> Self {
        Self(vec)
    }
}

impl<const N: usize, const D: usize> Collection<N, D> {
    pub(in crate::core::integration_step) const fn empty() -> Self {
        Self(Vec::new())
    }

    pub(in crate::core::integration_step) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(in crate::core::integration_step) fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(in crate::core::integration_step) fn iter(&self) -> impl Iterator<Item = &Variant<N, D>> {
        self.0.iter()
    }

    pub(in crate::core::integration_step) fn push(&mut self, data: Variant<N, D>) {
        self.0.push(data);
    }

    pub(crate) fn transmute<const A: usize, const B: usize>(self) -> Collection<A, B> {
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
