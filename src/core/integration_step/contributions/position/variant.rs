use super::{
    core::{Move, PhysicalQuantityKind},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
    Abstraction, Collection, DtFraction,
};

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
    pub(in super::super) fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::StartPosition { .. } => PhysicalQuantityKind::Position,
            Self::VelocityDt { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub fn evaluate_for(&self, step: &Step) -> Move {
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

    pub fn abstraction_scaled_for<'a>(&'a self, step: &'a Step, scale: f32) -> Abstraction<'a> {
        Abstraction::new(step, self.transmute(), scale)
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
