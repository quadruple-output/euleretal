use super::{
    core::{Fraction, Move, PhysicalQuantityKind, Position, Velocity},
    AccelerationRef, IntegrationStep, PositionRef, VelocityRef,
};

pub struct PositionContribution<'a> {
    step: &'a IntegrationStep,
    data: &'a PositionContributionData,
}

pub struct VelocityContribution<'a> {
    step: &'a IntegrationStep,
    data: &'a VelocityContributionData,
}

pub(super) enum PositionContributionData {
    StartPosition {
        s_ref: PositionRef,
    },
    VelocityDt {
        factor: f32,
        v_ref: VelocityRef,
        dt_fraction: Fraction,
    },
    AccelerationDtDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

pub(super) enum VelocityContributionData {
    Velocity {
        v_ref: VelocityRef,
    },
    AccelerationDt {
        factor: f32,
        a_ref: AccelerationRef,
        dt_fraction: Fraction,
    },
}

impl<'a> PositionContribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            PositionContributionData::StartPosition { s_ref } => step[*s_ref].s,
            PositionContributionData::VelocityDt { v_ref, .. } => {
                step[step[*v_ref].sampling_position].s
            }
            PositionContributionData::AccelerationDtDt { a_ref, .. } => {
                step[step[*a_ref].sampling_position].s
            }
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Option<Move> {
        match self.data {
            PositionContributionData::StartPosition { .. } => None,
            _ => Some(self.data.evaluate_for(self.step)),
        }
    }
}

impl<'a> VelocityContribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.step;
        match self.data {
            VelocityContributionData::Velocity { v_ref, .. } => {
                step[step[*v_ref].sampling_position].s
            }
            VelocityContributionData::AccelerationDt { a_ref, .. } => {
                step[step[*a_ref].sampling_position].s
            }
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.data.kind()
    }

    pub fn vector(&self) -> Velocity {
        self.data.evaluate_for(self.step)
    }
}

impl PositionContributionData {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::StartPosition { .. } => PhysicalQuantityKind::Position,
            Self::VelocityDt { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(super) fn evaluate_for(&self, step: &IntegrationStep) -> Move {
        match *self {
            Self::StartPosition { s_ref: sref } => step[sref].s.into(),
            Self::VelocityDt {
                factor,
                v_ref,
                dt_fraction,
            } => factor * &step[v_ref] * dt_fraction * step.dt(),
            Self::AccelerationDtDt {
                factor,
                a_ref,
                dt_fraction,
            } => factor * &step[a_ref] * (dt_fraction * step.dt()) * (dt_fraction * step.dt()),
        }
    }

    pub(super) fn public_for<'a>(&'a self, step: &'a IntegrationStep) -> PositionContribution<'a> {
        PositionContribution { step, data: self }
    }
}

impl VelocityContributionData {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::Velocity { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    pub(super) fn evaluate_for(&self, step: &IntegrationStep) -> Velocity {
        match *self {
            Self::Velocity { v_ref: vref } => step[vref].v,
            Self::AccelerationDt {
                factor,
                a_ref,
                dt_fraction,
            } => factor * &step[a_ref] * dt_fraction * step.dt(),
        }
    }

    pub(super) fn public_for<'a>(&'a self, step: &'a IntegrationStep) -> VelocityContribution<'a> {
        VelocityContribution { step, data: self }
    }
}
