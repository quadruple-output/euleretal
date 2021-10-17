use super::{
    core::{PhysicalQuantityKind, Velocity},
    position,
    step::{AccelerationRef, Step, VelocityRef},
    Abstraction, Collection, DtFraction,
};

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

    pub fn abstraction_scaled_for<'a>(&'a self, step: &'a Step, scale: f32) -> Abstraction<'a> {
        Abstraction::new(step, self.transmute(), scale)
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
