use ::std::ops::{AddAssign, Mul};

use super::{import::R32, Acceleration, Duration, Fraction, Position, Velocity};

/// These are the relevant derivatives of s, the position in space.
pub enum QuantityKind {
    Position,
    Velocity,
    Acceleration,
}

#[derive(Clone, Default)]
pub struct ComputedPosition {
    s: Position,
    pub contributions: Vec<PositionContribution>,
}

#[derive(Clone, Default)]
pub struct ComputedVelocity {
    v: Velocity,
    pub contributions: Vec<VelocityContribution>,
}

#[derive(Clone, Default)]
pub struct ComputedAcceleration {
    a: Acceleration,
    pub contributions: Vec<AccelerationContribution>,
}

// todo: rename to `Translation...`
#[derive(Clone)]
pub enum PositionContribution {
    StartPosition {
        sampling_position: ComputedPosition,
    },
    VelocityDt {
        sampling_position: ComputedPosition,
        factor: R32,
        v: ComputedVelocity,
        dt: Duration,
        dt_fraction: Fraction,
    },
    AccelerationDtDt {
        sampling_position: ComputedPosition,
        factor: R32,
        a: ComputedAcceleration,
        dt: Duration,
        dt_fraction: Fraction,
    },
}

#[derive(Clone)]
pub enum VelocityContribution {
    Velocity {
        sampling_position: ComputedPosition,
        v: ComputedVelocity,
    },
    AccelerationDt {
        sampling_position: ComputedPosition,
        factor: R32,
        a: ComputedAcceleration,
        dt: Duration,
        dt_fraction: Fraction,
    },
}

#[derive(Clone)]
pub enum AccelerationContribution {
    Acceleration {
        sampling_position: ComputedPosition,
        a: ComputedAcceleration,
    },
}

impl From<Vec<PositionContribution>> for ComputedPosition {
    fn from(contributions: Vec<PositionContribution>) -> Self {
        let mut s = Position::ZERO;
        Accumulator(&contributions).accumulate(&mut s);
        Self { s, contributions }
    }
}

impl From<Position> for ComputedPosition {
    fn from(s: Position) -> Self {
        Self {
            s,
            contributions: Vec::new(),
        }
    }
}

impl ComputedPosition {
    pub fn as_position(&self) -> Position {
        self.s
    }
}

impl From<Vec<VelocityContribution>> for ComputedVelocity {
    fn from(contributions: Vec<VelocityContribution>) -> Self {
        let mut v = Velocity::ZERO;
        Accumulator(&contributions).accumulate(&mut v);
        Self { v, contributions }
    }
}

impl From<Velocity> for ComputedVelocity {
    fn from(v: Velocity) -> Self {
        Self {
            v,
            contributions: Vec::new(),
        }
    }
}

impl ComputedVelocity {
    pub fn as_velocity(&self) -> Velocity {
        self.v
    }
}

impl From<Vec<AccelerationContribution>> for ComputedAcceleration {
    fn from(contributions: Vec<AccelerationContribution>) -> Self {
        let mut a = Acceleration::ZERO;
        Accumulator(&contributions).accumulate(&mut a);
        Self { a, contributions }
    }
}

impl From<Acceleration> for ComputedAcceleration {
    fn from(a: Acceleration) -> Self {
        Self {
            a,
            contributions: Vec::new(),
        }
    }
}

impl ComputedAcceleration {
    pub fn as_acceleration(&self) -> Acceleration {
        self.a
    }
}

pub trait Contribution {
    type Quantity: AddAssign;

    fn sampling_position(&self) -> Position;
    fn absolute(&self) -> Option<Self::Quantity>;
    fn delta(&self) -> Option<Self::Quantity>;
    fn base_quantity(&self) -> QuantityKind;
}

impl Contribution for PositionContribution {
    type Quantity = Position;

    fn sampling_position(&self) -> Position {
        match &self {
            Self::StartPosition { sampling_position }
            | Self::VelocityDt {
                sampling_position, ..
            }
            | Self::AccelerationDtDt {
                sampling_position, ..
            } => sampling_position.as_position(),
        }
    }

    fn absolute(&self) -> Option<Position> {
        if let Self::StartPosition { sampling_position } = &self {
            Some(sampling_position.as_position())
        } else {
            None
        }
    }

    fn delta(&self) -> Option<Position> {
        match self {
            Self::StartPosition { .. } => None,
            Self::VelocityDt {
                factor,
                v,
                dt,
                dt_fraction,
                ..
            } => Some(factor * v * dt * dt_fraction),
            Self::AccelerationDtDt {
                factor,
                a,
                dt,
                dt_fraction,
                ..
            } => Some(factor * a * (dt * dt_fraction) * (dt * dt_fraction)),
        }
    }

    fn base_quantity(&self) -> QuantityKind {
        match self {
            Self::StartPosition { .. } => QuantityKind::Position,
            Self::VelocityDt { .. } => QuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => QuantityKind::Acceleration,
        }
    }
}

impl Contribution for VelocityContribution {
    type Quantity = Velocity;

    fn sampling_position(&self) -> Position {
        match &self {
            VelocityContribution::Velocity {
                sampling_position, ..
            }
            | VelocityContribution::AccelerationDt {
                sampling_position, ..
            } => sampling_position.as_position(),
        }
    }

    fn absolute(&self) -> Option<Velocity> {
        match self {
            Self::Velocity { v, .. } => Some(v.as_velocity()),
            Self::AccelerationDt { .. } => None,
        }
    }

    fn delta(&self) -> Option<Velocity> {
        match self {
            Self::Velocity { .. } => None,
            Self::AccelerationDt {
                factor,
                a,
                dt,
                dt_fraction,
                ..
            } => Some(factor * a * dt * dt_fraction),
        }
    }

    fn base_quantity(&self) -> QuantityKind {
        match self {
            Self::Velocity { .. } => QuantityKind::Velocity,
            Self::AccelerationDt { .. } => QuantityKind::Acceleration,
        }
    }
}

impl Contribution for AccelerationContribution {
    type Quantity = Acceleration;

    fn sampling_position(&self) -> Acceleration {
        match self {
            Self::Acceleration {
                sampling_position, ..
            } => sampling_position.as_position(),
        }
    }

    fn absolute(&self) -> Option<Acceleration> {
        match self {
            Self::Acceleration { a, .. } => Some(a.as_acceleration()),
        }
    }

    fn delta(&self) -> Option<Acceleration> {
        None
    }

    fn base_quantity(&self) -> QuantityKind {
        QuantityKind::Acceleration
    }
}

impl Mul<&ComputedVelocity> for R32 {
    type Output = Velocity;

    fn mul(self, rhs: &ComputedVelocity) -> Self::Output {
        self.into_inner() * rhs.v
    }
}

impl Mul<&ComputedVelocity> for &R32 {
    type Output = Velocity;

    fn mul(self, rhs: &ComputedVelocity) -> Self::Output {
        self.into_inner() * rhs.v
    }
}

impl Mul<&ComputedAcceleration> for R32 {
    type Output = Acceleration;

    fn mul(self, rhs: &ComputedAcceleration) -> Self::Output {
        self.into_inner() * rhs.a
    }
}

impl Mul<&ComputedAcceleration> for &R32 {
    type Output = Acceleration;

    fn mul(self, rhs: &ComputedAcceleration) -> Self::Output {
        self.into_inner() * rhs.a
    }
}

struct Accumulator<'a, Contrib: Contribution>(&'a Vec<Contrib>);

impl<'a, Contrib: Contribution> Accumulator<'a, Contrib> {
    fn accumulate(self, accu: &mut Contrib::Quantity) {
        let mut got_absolute = false;
        for contrib in self.0 {
            if let Some(absolute) = contrib.absolute() {
                if got_absolute {
                    todo!("Implement weighted average for multiple sampling positions");
                }
                got_absolute = true;
                *accu += absolute;
            } else {
                *accu += contrib.delta().unwrap();
            }
        }
    }
}
