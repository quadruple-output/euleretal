use super::{
    import::R32, Acceleration, AccelerationField, Duration, Fraction, Integrator, Position,
    StartCondition, Velocity,
};
use ::std::ops::Mul;

pub struct IntegrationStep {
    dt: Duration,
    all_positions: Vec<ComputedPositionInternal>,
    all_velocities: Vec<ComputedVelocityInternal>,
    all_accelerations: Vec<ComputedAccelerationInternal>,
}

impl IntegrationStep {
    pub fn new(integrator: &dyn Integrator, dt: Duration) -> Self {
        Self {
            dt,
            all_positions: Vec::with_capacity(integrator.expected_positions_for_step()),
            all_velocities: Vec::with_capacity(integrator.expected_velocities_for_step()),
            all_accelerations: Vec::with_capacity(integrator.expected_accelerations_for_step()),
        }
    }

    pub fn start_position(&mut self, s: Position) -> PositionRef {
        PositionRef::new(
            self.computed_position(ComputedPositionInternal {
                s,
                contributions: Vec::new(),
            }),
            self,
        )
    }

    pub fn start_velocity(&mut self, v: Velocity, sampling_position: PositionRef) -> VelocityRef {
        VelocityRef::new(
            self.computed_velocity(ComputedVelocityInternal {
                v,
                sampling_position: sampling_position.0,
                contributions: Vec::new(),
            }),
            self,
        )
    }

    pub fn start_acceleration(
        &mut self,
        a: Acceleration,
        sampling_position: PositionRef,
    ) -> AccelerationRef {
        AccelerationRef::new(
            self.computed_acceleration(ComputedAccelerationInternal {
                a,
                sampling_position: sampling_position.0,
                //contributions: Vec::new(),
            }),
            self,
        )
    }

    pub fn initial_condition(&mut self, p: &StartCondition) -> ConditionRef {
        let sref = self.start_position(p.position);
        ConditionRef {
            s: sref,
            v: self.start_velocity(p.velocity, sref),
            a: self.start_acceleration(p.acceleration, sref),
        }
    }

    pub fn compute_position(&mut self, dt_fraction: Fraction) -> PositionBuilder {
        PositionBuilder::new(self, dt_fraction)
    }

    pub fn compute_velocity(
        &mut self,
        dt_fraction: Fraction,
        sref: PositionRef,
    ) -> VelocityBuilder {
        VelocityBuilder::new(self, dt_fraction, sref)
    }

    pub fn compute_acceleration_at(
        &mut self,
        sref: PositionRef,
        a: &dyn AccelerationField,
    ) -> AccelerationRef {
        let aref_int = self.computed_acceleration(ComputedAccelerationInternal {
            a: a.value_at(self.position_internal(sref.0).s),
            sampling_position: sref.0,
        });
        AccelerationRef(aref_int, self)
    }

    pub fn position(&self, pref: PositionRef) -> ComputedPosition {
        ComputedPosition {
            step: self,
            internal: &self.position_internal(pref.0),
        }
    }

    pub fn velocity(&self, pref: VelocityRef) -> ComputedVelocity {
        ComputedVelocity {
            step: self,
            internal: &self.velocity_internal(pref.0),
        }
    }

    pub fn acceleration(&self, pref: AccelerationRef) -> ComputedAcceleration {
        ComputedAcceleration {
            step: self,
            internal: &self.acceleration_internal(pref.0),
        }
    }

    fn computed_position(&mut self, p: ComputedPositionInternal) -> PositionRefInternal {
        self.all_positions.push(p);
        PositionRefInternal(self.all_positions.len() - 1)
    }

    fn computed_velocity(&mut self, p: ComputedVelocityInternal) -> VelocityRefInternal {
        self.all_velocities.push(p);
        VelocityRefInternal(self.all_velocities.len() - 1)
    }

    fn computed_acceleration(
        &mut self,
        p: ComputedAccelerationInternal,
    ) -> AccelerationRefInternal {
        self.all_accelerations.push(p);
        AccelerationRefInternal(self.all_accelerations.len() - 1)
    }

    fn position_internal(&self, pref: PositionRefInternal) -> &ComputedPositionInternal {
        &self.all_positions[pref.0]
    }

    fn velocity_internal(&self, pref: VelocityRefInternal) -> &ComputedVelocityInternal {
        &self.all_velocities[pref.0]
    }

    fn acceleration_internal(
        &self,
        pref: AccelerationRefInternal,
    ) -> &ComputedAccelerationInternal {
        &self.all_accelerations[pref.0]
    }
}

#[derive(Clone, Copy)]
struct PositionRefInternal(usize);
#[derive(Clone, Copy)]
pub struct PositionRef(PositionRefInternal, *mut IntegrationStep);
impl PositionRef {
    fn new(pref: PositionRefInternal, step: &mut IntegrationStep) -> Self {
        Self(pref, step)
    }
}

#[derive(Clone, Copy)]
struct VelocityRefInternal(usize);
#[derive(Clone, Copy)]
pub struct VelocityRef(VelocityRefInternal, *mut IntegrationStep);
impl VelocityRef {
    fn new(vref: VelocityRefInternal, step: &mut IntegrationStep) -> Self {
        Self(vref, step)
    }
}

#[derive(Clone, Copy)]
struct AccelerationRefInternal(usize);
#[derive(Clone, Copy)]
pub struct AccelerationRef(AccelerationRefInternal, *mut IntegrationStep);
impl AccelerationRef {
    fn new(aref: AccelerationRefInternal, step: &mut IntegrationStep) -> Self {
        Self(aref, step)
    }
}

struct ComputedPositionInternal {
    s: Position,
    contributions: Vec<PositionContributionInternal>,
}

struct ComputedVelocityInternal {
    v: Velocity,
    sampling_position: PositionRefInternal,
    contributions: Vec<VelocityContributionInternal>,
}

struct ComputedAccelerationInternal {
    a: Acceleration,
    sampling_position: PositionRefInternal,
    //contributions: Vec<AccelerationContributionInternal>,
}

enum PositionContributionInternal {
    StartPosition {
        sref: PositionRefInternal,
    },
    VelocityDt {
        factor: R32,
        vref: VelocityRefInternal,
        dt_fraction: Fraction,
    },
    AccelerationDtDt {
        factor: R32,
        aref: AccelerationRefInternal,
        dt_fraction: Fraction,
    },
}

enum VelocityContributionInternal {
    Velocity {
        vref: VelocityRefInternal,
    },
    AccelerationDt {
        factor: R32,
        aref: AccelerationRefInternal,
        dt_fraction: Fraction,
    },
}

//enum AccelerationContributionInternal {
//    Acceleration { aref: AccelerationRef },
//}

pub struct ComputedPosition<'a> {
    step: &'a IntegrationStep,
    internal: &'a ComputedPositionInternal,
}

pub struct PositionContribution<'a> {
    position: &'a ComputedPosition<'a>,
    internal: &'a PositionContributionInternal,
}

pub struct ComputedVelocity<'a> {
    step: &'a IntegrationStep,
    internal: &'a ComputedVelocityInternal,
}

pub struct VelocityContribution<'a> {
    position: &'a ComputedVelocity<'a>,
    internal: &'a VelocityContributionInternal,
}

pub struct ComputedAcceleration<'a> {
    step: &'a IntegrationStep,
    internal: &'a ComputedAccelerationInternal,
}

#[derive(Clone, Copy)]
pub struct ConditionRef {
    pub s: PositionRef,
    pub v: VelocityRef,
    pub a: AccelerationRef,
}

pub struct PositionBuilder<'a> {
    step: &'a mut IntegrationStep,
    dt_fraction: Fraction,
    contributions: Vec<PositionContributionInternal>,
}

impl<'a> PositionBuilder<'a> {
    fn new(step: &'a mut IntegrationStep, dt_fraction: Fraction) -> Self {
        Self {
            step,
            dt_fraction,
            // most of the times there will be 3 contributions:
            contributions: Vec::with_capacity(3),
        }
    }

    pub fn based_on(mut self, sref: PositionRef) -> Self {
        self.contributions
            .push(PositionContributionInternal::StartPosition { sref: sref.0 });
        self
    }

    pub fn add_velocity_dt(mut self, vref: VelocityRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionInternal::VelocityDt {
                factor: factor.into(),
                vref: vref.0,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn add_acceleration_dt_dt(mut self, aref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionInternal::AccelerationDtDt {
                factor: factor.into(),
                aref: aref.0,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> PositionRef {
        let mut s = Position::ZERO;
        for contrib in &self.contributions {
            s += contrib.evaluate_for(self.step);
        }
        PositionRef::new(
            self.step.computed_position(ComputedPositionInternal {
                s,
                contributions: self.contributions,
            }),
            self.step,
        )
    }
}

pub struct VelocityBuilder<'a> {
    step: &'a mut IntegrationStep,
    dt_fraction: Fraction,
    sref: PositionRefInternal,
    contributions: Vec<VelocityContributionInternal>,
}

impl<'a> VelocityBuilder<'a> {
    fn new(step: &'a mut IntegrationStep, dt_fraction: Fraction, sref: PositionRef) -> Self {
        Self {
            step,
            dt_fraction,
            sref: sref.0,
            // most of the times there will be 2 contributions:
            contributions: Vec::with_capacity(2),
        }
    }

    pub fn based_on(mut self, vref: VelocityRef) -> Self {
        self.contributions
            .push(VelocityContributionInternal::Velocity { vref: vref.0 });
        self
    }

    pub fn add_acceleration_dt(mut self, aref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(VelocityContributionInternal::AccelerationDt {
                factor: factor.into(),
                aref: aref.0,
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> VelocityRef {
        let mut v = Velocity::ZERO;
        for contrib in &self.contributions {
            v += contrib.evaluate_for(self.step);
        }
        VelocityRef::new(
            self.step.computed_velocity(ComputedVelocityInternal {
                v,
                sampling_position: self.sref,
                contributions: self.contributions,
            }),
            self.step,
        )
    }
}

impl PositionContributionInternal {
    fn evaluate_for(&self, step: &IntegrationStep) -> Position {
        match self {
            Self::StartPosition { sref } => step.position_internal(*sref).s,
            Self::VelocityDt {
                factor,
                vref,
                dt_fraction,
            } => {
                let v = step.velocity_internal(*vref);
                factor * v * dt_fraction * step.dt
            }
            Self::AccelerationDtDt {
                factor,
                aref,
                dt_fraction,
            } => {
                let a = step.acceleration_internal(*aref);
                factor * a * (dt_fraction * step.dt) * (dt_fraction * step.dt)
            }
        }
    }
}

impl VelocityContributionInternal {
    fn evaluate_for(&self, step: &IntegrationStep) -> Velocity {
        match self {
            Self::Velocity { vref } => step.velocity_internal(*vref).v,
            Self::AccelerationDt {
                factor,
                aref,
                dt_fraction,
            } => {
                let a = step.acceleration_internal(*aref);
                factor * a * dt_fraction * step.dt
            }
        }
    }
}

impl Mul<&ComputedVelocityInternal> for &R32 {
    type Output = Velocity;

    fn mul(self, rhs: &ComputedVelocityInternal) -> Self::Output {
        self.into_inner() * rhs.v
    }
}

impl Mul<&ComputedAccelerationInternal> for &R32 {
    type Output = Acceleration;

    fn mul(self, rhs: &ComputedAccelerationInternal) -> Self::Output {
        self.into_inner() * rhs.a
    }
}
