use super::{
    dt_fraction::{DtFraction, FractionSpec},
    position,
    step::{AccelerationRef, Step, VelocityRef},
    Abstraction, Collection,
};
use crate::{Fraction, PhysicalQuantityKind, Velocity};

#[derive(Clone, Copy, Debug)]
pub enum Variant<FRACTION: FractionSpec> {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: FRACTION,
    },
}

impl<F: FractionSpec> From<VelocityRef> for Variant<F> {
    fn from(v_ref: VelocityRef) -> Self {
        Self::Velocity { v_ref }
    }
}

impl<F: FractionSpec> Variant<F> {
    pub fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::Velocity { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub fn evaluate_for(&self, step: &Step) -> Velocity {
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

    pub fn abstraction_scaled_for<'a>(
        &'a self,
        step: &'a Step,
        fraction: Fraction,
    ) -> Abstraction<'a> {
        Abstraction::new(
            step,
            match *self {
                Variant::Velocity { v_ref } => Variant::Velocity { v_ref },
                Variant::AccelerationDt {
                    factor,
                    a_ref,
                    dt_fraction: _,
                } => Variant::AccelerationDt {
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

    fn add(self, rhs: Variant<DtFraction<N, D>>) -> Self::Output {
        vec![self, rhs].into()
    }
}

impl<const N: usize, const D: usize> std::ops::Mul<DtFraction<N, D>> for Variant<DtFraction<N, D>> {
    type Output = position::Variant<DtFraction<N, D>>;

    fn mul(self, fraction: DtFraction<N, D>) -> Self::Output {
        match self {
            Variant::Velocity { v_ref } => position::Variant::VelocityDt {
                factor: 1.,
                v_ref,
                dt_fraction: fraction,
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
