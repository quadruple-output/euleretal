use super::{
    core::{Fraction, PhysicalQuantityKind, Position, Velocity},
    position,
    step::{AccelerationRef, Step, VelocityRef},
    DtFraction,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: &'a Variant,
}

impl<'a> Abstraction<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.variant {
            Variant::Velocity { v_ref, .. } => step[step[*v_ref].sampling_position].s,
            Variant::AccelerationDt { a_ref, .. } => step[step[*a_ref].sampling_position].s,
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.variant.kind()
    }

    pub fn vector(&self) -> Velocity {
        self.variant.evaluate_for(self.step)
    }
}

pub enum Variant {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

impl From<VelocityRef> for Variant {
    fn from(v_ref: VelocityRef) -> Self {
        Self::Velocity { v_ref }
    }
}

impl Variant {
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

impl std::ops::Mul<DtFraction> for Variant {
    type Output = position::Variant;

    fn mul(self, dt_fraction: DtFraction) -> Self::Output {
        match self {
            Variant::Velocity { v_ref } => {
                position::Variant::VelocityDt {
                    factor: 1., //todo
                    v_ref,
                    dt_fraction: dt_fraction.into(),
                }
            }
            Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction: dt_fraction_lhs,
            } => {
                // todo: cannot handle `a * dt * dt_2` where dt != dt_2
                debug_assert_eq!(dt_fraction_lhs, dt_fraction.into());
                position::Variant::AccelerationDtDt {
                    factor,
                    a_ref,
                    dt_fraction: dt_fraction_lhs,
                }
            }
        }
    }
}
