use super::{
    core::{Move, PhysicalQuantityKind, Position},
    step::Step,
    Variant,
};

pub struct Abstraction<'a> {
    step: &'a Step,
    // Abstraction cannot be parameterized, so we move the static fraction to a component
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
