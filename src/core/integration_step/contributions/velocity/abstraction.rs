use super::{
    core::{PhysicalQuantityKind, Position, Velocity},
    step::Step,
    Variant,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    variant: Variant<1, 1>,
    variant_scale: f32,
}

impl<'a> Abstraction<'a> {
    pub fn new(step: &'a Step, variant: Variant<1, 1>, variant_scale: f32) -> Self {
        Self {
            step,
            variant,
            variant_scale,
        }
    }

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
