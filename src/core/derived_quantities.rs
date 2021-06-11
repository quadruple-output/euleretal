use std::ops::Mul;

use super::{import::R32, Acceleration, Duration, Fraction, Position, Velocity};

#[derive(Clone, Default)]
pub struct DerivedPosition {
    s: Position,
    pub contributions: Vec<PositionContribution>,
}

#[derive(Clone, Default)]
pub struct DerivedVelocity {
    v: Velocity,
    pub contributions: Vec<VelocityContribution>,
}

#[derive(Clone, Default)]
pub struct DerivedAcceleration {
    a: Acceleration,
    pub contributions: Vec<AccelerationContribution>,
}

// todo: rename to `Translation...`
#[derive(Clone)]
pub struct PositionContribution {
    pub sampling_position: DerivedPosition,
    pub quantity: PositionFragment,
}

#[derive(Clone)]
pub struct VelocityContribution {
    pub sampling_position: DerivedPosition,
    pub quantity: VelocityFragment,
}

#[derive(Clone)]
pub struct AccelerationContribution {
    pub sampling_position: DerivedPosition,
    pub quantity: AccelerationFragment,
}

#[derive(Clone)]
pub enum PositionFragment {
    Position {
        // this is odd. todo: merge PositionContribution and PositionFragment into a single enum.
    },
    VelocityDt {
        factor: R32,
        v: DerivedVelocity,
        dt: Duration,
        dt_fraction: Fraction,
    },
    AccelerationDtDt {
        factor: R32,
        a: DerivedAcceleration,
        dt: Duration,
        dt_fraction: Fraction,
    },
}

#[derive(Clone)]
pub enum VelocityFragment {
    Velocity {
        v: DerivedVelocity,
    },
    AccelerationDt {
        factor: R32,
        a: DerivedAcceleration,
        dt: Duration,
        dt_fraction: Fraction,
    },
}

#[derive(Clone)]
pub enum AccelerationFragment {
    Acceleration { a: DerivedAcceleration },
}

impl From<Vec<PositionContribution>> for DerivedPosition {
    fn from(contributions: Vec<PositionContribution>) -> Self {
        let mut s = Position::ZERO;
        for contrib in &contributions {
            s += contrib.eff_position();
        }
        Self { s, contributions }
    }
}

impl From<Vec<VelocityContribution>> for DerivedVelocity {
    fn from(contributions: Vec<VelocityContribution>) -> Self {
        let mut v = Velocity::ZERO;
        for contrib in &contributions {
            v += contrib.eff_velocity();
        }
        Self { v, contributions }
    }
}

impl From<Vec<AccelerationContribution>> for DerivedAcceleration {
    fn from(contributions: Vec<AccelerationContribution>) -> Self {
        let mut a = Acceleration::ZERO;
        for contrib in &contributions {
            a += contrib.eff_acceleration();
        }
        Self { a, contributions }
    }
}

impl From<&DerivedVelocity> for Velocity {
    fn from(dv: &DerivedVelocity) -> Self {
        dv.v
    }
}

impl From<&DerivedPosition> for Position {
    fn from(dp: &DerivedPosition) -> Self {
        dp.s
    }
}

impl From<&DerivedAcceleration> for Acceleration {
    fn from(da: &DerivedAcceleration) -> Self {
        da.a
    }
}

impl From<Position> for DerivedPosition {
    fn from(s: Position) -> Self {
        Self {
            s,
            contributions: Vec::new(),
        }
    }
}

impl From<Velocity> for DerivedVelocity {
    fn from(v: Velocity) -> Self {
        Self {
            v,
            contributions: Vec::new(),
        }
    }
}

impl From<Acceleration> for DerivedAcceleration {
    fn from(a: Acceleration) -> Self {
        Self {
            a,
            contributions: Vec::new(),
        }
    }
}

impl Mul<&DerivedVelocity> for R32 {
    type Output = Velocity;

    fn mul(self, rhs: &DerivedVelocity) -> Self::Output {
        self.into_inner() * rhs.v
    }
}

impl Mul<&DerivedAcceleration> for R32 {
    type Output = Acceleration;

    fn mul(self, rhs: &DerivedAcceleration) -> Self::Output {
        self.into_inner() * rhs.a
    }
}

impl PositionContribution {
    pub fn eff_position(&self) -> Position {
        match &self.quantity {
            PositionFragment::Position {} => (&self.sampling_position).into(),
            PositionFragment::VelocityDt {
                factor,
                v,
                dt,
                dt_fraction,
            } => *factor * v * *dt * *dt_fraction,
            PositionFragment::AccelerationDtDt {
                factor,
                a,
                dt,
                dt_fraction,
            } => *factor * a * (*dt * *dt_fraction) * (*dt * *dt_fraction), // todo: a*dt*dt should create a Position, not a generic Vec3
        }
    }
}

impl VelocityContribution {
    pub fn eff_velocity(&self) -> Velocity {
        match &self.quantity {
            VelocityFragment::Velocity { v } => v.into(),
            VelocityFragment::AccelerationDt {
                factor,
                a,
                dt,
                dt_fraction,
            } => *factor * a * *dt * *dt_fraction,
        }
    }
}

impl AccelerationContribution {
    pub fn eff_acceleration(&self) -> Acceleration {
        match &self.quantity {
            AccelerationFragment::Acceleration { a } => a.into(),
        }
    }
}
