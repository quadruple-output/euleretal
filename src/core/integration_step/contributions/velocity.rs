use super::{
    core::{DtFraction, PhysicalQuantityKind, Position, Velocity},
    position,
    step::{AccelerationRef, Step, VelocityRef},
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

#[derive(Clone, Copy)]
pub enum Variant {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: DtFraction,
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
            } => {
                let dt = dt_fraction * step.dt();
                let a = step[a_ref].a;
                factor * a * dt
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

    fn add(self, rhs: Variant) -> Self::Output {
        vec![self, rhs].into()
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
                    dt_fraction,
                }
            }
            Variant::AccelerationDt {
                factor,
                a_ref,
                dt_fraction: dt_fraction_lhs,
            } => {
                // todo: cannot handle `a * dt * dt_2` where dt != dt_2
                debug_assert_eq!(dt_fraction_lhs, dt_fraction);
                position::Variant::AccelerationDtDt {
                    factor,
                    a_ref,
                    dt_fraction: dt_fraction_lhs,
                }
            }
        }
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

    pub(in crate::core::integration_step) fn is_empty(&self) -> bool {
        self.0.is_empty()
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
