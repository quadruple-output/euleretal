use super::{
    dt_fraction::{DtFraction, FractionSpec},
    step::{AccelerationRef, PositionRef, Step, VelocityRef},
    Abstraction, Collection,
};
use crate::{Fraction, Move, PhysicalQuantityKind};

#[derive(Clone, Copy, Debug)]
pub enum Variant<FRACTION: FractionSpec> {
    //pub enum Variant<const N: usize, const D: usize> {
    StartPosition {
        s_ref: PositionRef,
    },
    VelocityDt {
        factor: f32,
        v_ref: VelocityRef,
        dt_fraction: FRACTION,
    },
    AccelerationDtDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: FRACTION,
    },
}

impl<F: FractionSpec> From<PositionRef> for Variant<F> {
    fn from(s_ref: PositionRef) -> Self {
        Self::StartPosition { s_ref }
    }
}

impl<F: FractionSpec> Variant<F> {
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

    pub fn abstraction_scaled_for<'a>(
        &'a self,
        step: &'a Step,
        fraction: Fraction,
    ) -> Abstraction<'a> {
        Abstraction::new(
            step,
            match *self {
                Variant::StartPosition { s_ref } => Variant::StartPosition { s_ref },
                Variant::VelocityDt {
                    factor,
                    v_ref,
                    dt_fraction: _,
                } => Variant::VelocityDt {
                    factor,
                    v_ref,
                    dt_fraction: fraction,
                },
                Variant::AccelerationDtDt {
                    factor,
                    a_ref,
                    dt_fraction: _,
                } => Variant::AccelerationDtDt {
                    factor,
                    a_ref,
                    dt_fraction: fraction,
                },
            },
        )
    }
}

impl<const N: usize, const D: usize> std::ops::Add for Variant<DtFraction<N, D>> {
    type Output = Collection<N, D>;

    fn add(self, rhs: Self) -> Self::Output {
        vec![self, rhs].into()
    }
}
