use super::{
    super::integration_step::{
        quantity_contributions,
        step::{AccelerationRef, PositionRef, VelocityRef},
        Step,
    },
    core::{self, Fraction},
};

pub struct Position<'a> {
    step: &'a mut Step,
    dt_fraction: Fraction,
    contributions: quantity_contributions::position::Collection,
}

impl<'a> Position<'a> {
    pub fn new(step: &'a mut Step, dt_fraction: Fraction) -> Self {
        Self {
            step,
            dt_fraction,
            // most of the times there will be 3 contributions:
            contributions: quantity_contributions::position::Collection::with_capacity(3),
        }
    }

    pub fn based_on(mut self, s_ref: PositionRef) -> Self {
        self.contributions
            .push(quantity_contributions::position::Variant::StartPosition { s_ref });
        self
    }

    pub fn add_velocity_dt(mut self, v_ref: VelocityRef, factor: f32) -> Self {
        self.contributions
            .push(quantity_contributions::position::Variant::VelocityDt {
                factor,
                v_ref,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn add_acceleration_dt_dt(mut self, a_ref: AccelerationRef, factor: f32) -> Self {
        self.contributions.push(
            quantity_contributions::position::Variant::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction: self.dt_fraction,
            },
        );
        self
    }

    pub fn create(self) -> PositionRef {
        let mut s = core::Position::origin();
        for contrib in self.contributions.iter() {
            s += contrib.evaluate_for(self.step);
        }
        self.step
            .add_computed_position(s, self.dt_fraction, self.contributions)
    }
}

pub struct PositionContribution {
    inner: quantity_contributions::position::Variant,
}

impl From<quantity_contributions::position::Variant> for PositionContribution {
    fn from(data: quantity_contributions::position::Variant) -> Self {
        Self { inner: data }
    }
}

impl From<PositionRef> for PositionContribution {
    fn from(s_ref: PositionRef) -> Self {
        quantity_contributions::position::Variant::StartPosition { s_ref }.into()
    }
}

impl From<PositionContribution> for quantity_contributions::position::Variant {
    fn from(p: PositionContribution) -> Self {
        p.inner
    }
}

impl std::ops::Add<PositionContribution> for PositionContribution {
    type Output = quantity_contributions::position::Collection;

    fn add(self, rhs: PositionContribution) -> Self::Output {
        quantity_contributions::position::Collection(vec![self.inner, rhs.inner])
    }
}

impl std::ops::Add<PositionContribution> for quantity_contributions::position::Collection {
    type Output = Self;

    fn add(self, rhs: PositionContribution) -> Self::Output {
        Self(self.0.into_iter().chain(Some(rhs.inner)).collect())
    }
}
