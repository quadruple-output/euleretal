mod public_refs {
    use super::{
        AccelerationRefInternal, IntegrationStep, PositionRefInternal, VelocityRefInternal,
    };

    #[derive(Clone, Copy)]
    pub struct PositionRef(PositionRefInternal, *const IntegrationStep);
    impl PositionRef {
        pub(super) fn new(pref: PositionRefInternal, step: &mut IntegrationStep) -> Self {
            Self(pref, step)
        }

        pub(super) fn internal_for(self, step: &IntegrationStep) -> PositionRefInternal {
            assert!(self.1 == step);
            self.0
        }
    }

    #[derive(Clone, Copy)]
    pub struct VelocityRef(VelocityRefInternal, *const IntegrationStep);
    impl VelocityRef {
        pub(super) fn new(vref: VelocityRefInternal, step: &mut IntegrationStep) -> Self {
            Self(vref, step)
        }

        pub(super) fn internal_for(self, step: &IntegrationStep) -> VelocityRefInternal {
            assert!(self.1 == step);
            self.0
        }
    }

    #[derive(Clone, Copy)]
    pub struct AccelerationRef(AccelerationRefInternal, *const IntegrationStep);
    impl AccelerationRef {
        pub(super) fn new(aref: AccelerationRefInternal, step: &mut IntegrationStep) -> Self {
            Self(aref, step)
        }

        pub(super) fn internal_for(self, step: &IntegrationStep) -> AccelerationRefInternal {
            assert!(self.1 == step);
            self.0
        }
    }
}

use super::{
    import::R32, integrator, Acceleration, AccelerationField, Duration, Fraction,
    PhysicalQuantityKind, Position, StartCondition, Velocity,
};
use ::std::ops::Mul;
pub use public_refs::{AccelerationRef, PositionRef, VelocityRef};

pub struct IntegrationStep {
    pub dt: Duration,
    all_positions: Vec<ComputedPositionInternal>,
    all_velocities: Vec<ComputedVelocityInternal>,
    all_accelerations: Vec<ComputedAccelerationInternal>,
    last_computed_position: Option<PositionRefInternal>,
    last_computed_velocity: Option<VelocityRefInternal>,
    acceleration_at_last_position: Option<AccelerationRefInternal>,
}

impl IntegrationStep {
    pub fn new(capacities: integrator::ExpectedCapacities, dt: Duration) -> Self {
        Self {
            dt,
            all_positions: Vec::with_capacity(capacities.positions + 1),
            all_velocities: Vec::with_capacity(capacities.velocities + 1),
            all_accelerations: Vec::with_capacity(capacities.accelerations + 1),
            last_computed_position: None,
            last_computed_velocity: None,
            acceleration_at_last_position: None,
        }
    }

    pub fn raw_from_condition(dt: Duration, s: Position, v: Velocity, a: Acceleration) -> Self {
        let mut result = Self {
            dt,
            all_positions: Vec::with_capacity(1),
            all_velocities: Vec::with_capacity(1),
            all_accelerations: Vec::with_capacity(1),
            last_computed_position: None,
            last_computed_velocity: None,
            acceleration_at_last_position: None,
        };
        let pref = result.add_computed_position(ComputedPositionInternal {
            s,
            contributions: Vec::new(),
        });
        result.last_computed_position = Some(pref);
        result.last_computed_velocity =
            Some(result.add_computed_velocity(ComputedVelocityInternal {
                v,
                sampling_position: pref,
                contributions: Vec::new(),
            }));
        result.acceleration_at_last_position = Some(result.add_computed_acceleration(
            ComputedAccelerationInternal {
                a,
                sampling_position: pref,
            },
        ));
        result
    }

    pub fn start_position(&mut self, s: Position) -> PositionRef {
        PositionRef::new(
            self.add_computed_position(ComputedPositionInternal {
                s,
                contributions: Vec::new(),
            }),
            self,
        )
    }

    pub fn start_velocity(&mut self, v: Velocity, sampling_position: PositionRef) -> VelocityRef {
        VelocityRef::new(
            self.add_computed_velocity(ComputedVelocityInternal {
                v,
                sampling_position: sampling_position.internal_for(self),
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
            self.add_computed_acceleration(ComputedAccelerationInternal {
                a,
                sampling_position: sampling_position.internal_for(self),
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

    pub fn next_condition(&self) -> Option<StartCondition> {
        if let (Some(pref), Some(vref), Some(aref)) = (
            self.last_computed_position,
            self.last_computed_velocity,
            self.acceleration_at_last_position,
        ) {
            Some(StartCondition {
                position: self.internal_get_position(pref).s,
                velocity: self.internal_get_velocity(vref).v,
                acceleration: self.internal_get_acceleration(aref).a,
            })
        } else {
            None
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
        let s_ref_int = sref.internal_for(self);
        let a_ref_int = self.add_computed_acceleration(ComputedAccelerationInternal {
            a: a.value_at(self.internal_get_position(s_ref_int).s),
            sampling_position: s_ref_int,
        });
        AccelerationRef::new(a_ref_int, self)
    }

    pub fn compute_acceleration_at_last_position(&mut self, a: &dyn AccelerationField) {
        let last_pref = self.last_computed_position.unwrap();
        self.acceleration_at_last_position = Some(self.add_computed_acceleration(
            ComputedAccelerationInternal {
                a: a.value_at(self.internal_get_position(last_pref).s),
                sampling_position: last_pref,
            },
        ));
    }

    pub fn get_position(&self, pref: PositionRef) -> ComputedPosition {
        ComputedPosition {
            step: self,
            internal: self.internal_get_position(pref.internal_for(self)),
        }
    }

    pub fn get_velocity(&self, pref: VelocityRef) -> ComputedVelocity {
        ComputedVelocity {
            step: self,
            internal: self.internal_get_velocity(pref.internal_for(self)),
        }
    }

    pub fn get_acceleration(&self, pref: AccelerationRef) -> ComputedAcceleration {
        ComputedAcceleration {
            step: self,
            internal: self.internal_get_acceleration(pref.internal_for(self)),
        }
    }

    pub fn last_computed_position(&self) -> ComputedPosition {
        ComputedPosition {
            step: self,
            internal: self.internal_get_position(self.last_computed_position.unwrap()),
        }
    }

    pub fn last_computed_velocity(&self) -> ComputedVelocity {
        ComputedVelocity {
            step: self,
            internal: self.internal_get_velocity(self.last_computed_velocity.unwrap()),
        }
    }

    pub fn last_s(&self) -> Position {
        self.internal_get_position(self.last_computed_position.unwrap())
            .s
    }

    pub fn last_v(&self) -> Velocity {
        self.internal_get_velocity(self.last_computed_velocity.unwrap())
            .v
    }

    pub fn positions_iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.all_positions.iter().map(|comp_pos| comp_pos.s)
    }

    pub fn velocities_iter(&self) -> impl Iterator<Item = (Position, Velocity)> + '_ {
        self.all_velocities.iter().map(move |comp_vel| {
            (
                self.internal_get_position(comp_vel.sampling_position).s,
                comp_vel.v,
            )
        })
    }

    pub fn accelerations_iter(&self) -> impl Iterator<Item = (Position, Acceleration)> + '_ {
        self.all_accelerations.iter().map(move |comp_acc| {
            (
                self.internal_get_position(comp_acc.sampling_position).s,
                comp_acc.a,
            )
        })
    }

    fn add_computed_position(&mut self, p: ComputedPositionInternal) -> PositionRefInternal {
        self.all_positions.push(p);
        PositionRefInternal(self.all_positions.len() - 1)
    }

    fn add_computed_velocity(&mut self, p: ComputedVelocityInternal) -> VelocityRefInternal {
        self.all_velocities.push(p);
        VelocityRefInternal(self.all_velocities.len() - 1)
    }

    fn add_computed_acceleration(
        &mut self,
        p: ComputedAccelerationInternal,
    ) -> AccelerationRefInternal {
        self.all_accelerations.push(p);
        AccelerationRefInternal(self.all_accelerations.len() - 1)
    }

    fn internal_get_position(&self, pref: PositionRefInternal) -> &ComputedPositionInternal {
        &self.all_positions[pref.0]
    }

    fn internal_get_velocity(&self, pref: VelocityRefInternal) -> &ComputedVelocityInternal {
        &self.all_velocities[pref.0]
    }

    fn internal_get_acceleration(
        &self,
        pref: AccelerationRefInternal,
    ) -> &ComputedAccelerationInternal {
        &self.all_accelerations[pref.0]
    }
}

#[derive(Clone, Copy)]
struct PositionRefInternal(usize);

#[derive(Clone, Copy)]
struct VelocityRefInternal(usize);

#[derive(Clone, Copy)]
struct AccelerationRefInternal(usize);

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

pub struct ComputedPosition<'a> {
    step: &'a IntegrationStep,
    internal: &'a ComputedPositionInternal,
}

impl<'a> ComputedPosition<'a> {
    pub fn s(&self) -> Position {
        self.internal.s
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = PositionContribution> {
        self.internal
            .contributions
            .iter()
            .map(move |contrib_int| PositionContribution {
                position: self,
                internal: contrib_int,
            })
    }
}

pub struct PositionContribution<'a> {
    position: &'a ComputedPosition<'a>,
    internal: &'a PositionContributionInternal,
}

impl<'a> PositionContribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.position.step;
        match self.internal {
            PositionContributionInternal::StartPosition { sref } => {
                step.internal_get_position(*sref).s
            }
            PositionContributionInternal::VelocityDt { vref, .. } => {
                step.internal_get_position(step.internal_get_velocity(*vref).sampling_position)
                    .s
            }
            PositionContributionInternal::AccelerationDtDt { aref, .. } => {
                step.internal_get_position(step.internal_get_acceleration(*aref).sampling_position)
                    .s
            }
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.internal.kind()
    }

    pub fn vector(&self) -> Option<Position> {
        match self.internal {
            PositionContributionInternal::StartPosition { .. } => None,
            _ => Some(self.internal.evaluate_for(self.position.step)),
        }
    }
}

pub struct ComputedVelocity<'a> {
    step: &'a IntegrationStep,
    internal: &'a ComputedVelocityInternal,
}

impl<'a> ComputedVelocity<'a> {
    pub fn v(&self) -> Velocity {
        self.internal.v
    }

    pub fn sampling_position(&self) -> Position {
        self.step
            .internal_get_position(self.internal.sampling_position)
            .s
    }

    pub fn contributions_iter(&self) -> impl Iterator<Item = VelocityContribution> {
        self.internal
            .contributions
            .iter()
            .map(move |contrib_int| VelocityContribution {
                velocity: self,
                internal: contrib_int,
            })
    }
}

pub struct VelocityContribution<'a> {
    velocity: &'a ComputedVelocity<'a>,
    internal: &'a VelocityContributionInternal,
}

impl<'a> VelocityContribution<'a> {
    pub fn sampling_position(&self) -> Position {
        let step = self.velocity.step;
        match self.internal {
            VelocityContributionInternal::Velocity { vref, .. } => {
                step.internal_get_position(step.internal_get_velocity(*vref).sampling_position)
                    .s
            }
            VelocityContributionInternal::AccelerationDt { aref, .. } => {
                step.internal_get_position(step.internal_get_acceleration(*aref).sampling_position)
                    .s
            }
        }
    }

    pub fn kind(&self) -> PhysicalQuantityKind {
        self.internal.kind()
    }

    pub fn vector(&self) -> Velocity {
        self.internal.evaluate_for(self.velocity.step)
    }
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
            .push(PositionContributionInternal::StartPosition {
                sref: sref.internal_for(self.step),
            });
        self
    }

    pub fn add_velocity_dt(mut self, vref: VelocityRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionInternal::VelocityDt {
                factor: factor.into(),
                vref: vref.internal_for(self.step),
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn add_acceleration_dt_dt(mut self, aref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(PositionContributionInternal::AccelerationDtDt {
                factor: factor.into(),
                aref: aref.internal_for(self.step),
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> PositionRef {
        let mut s = Position::zeros();
        for contrib in &self.contributions {
            s += contrib.evaluate_for(self.step);
        }
        let s_ref_internal = self.step.add_computed_position(ComputedPositionInternal {
            s,
            contributions: self.contributions,
        });
        self.step.last_computed_position = Some(s_ref_internal);
        PositionRef::new(s_ref_internal, self.step)
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
        let sref_internal = sref.internal_for(step);
        Self {
            step,
            dt_fraction,
            sref: sref_internal,
            // most of the times there will be 2 contributions:
            contributions: Vec::with_capacity(2),
        }
    }

    pub fn based_on(mut self, vref: VelocityRef) -> Self {
        self.contributions
            .push(VelocityContributionInternal::Velocity {
                vref: vref.internal_for(self.step),
            });
        self
    }

    pub fn add_acceleration_dt(mut self, aref: AccelerationRef, factor: f32) -> Self {
        self.contributions
            .push(VelocityContributionInternal::AccelerationDt {
                factor: factor.into(),
                aref: aref.internal_for(self.step),
                dt_fraction: self.dt_fraction,
            });
        self
    }

    pub fn create(self) -> VelocityRef {
        let mut v = Velocity::zeros();
        for contrib in &self.contributions {
            v += contrib.evaluate_for(self.step);
        }
        let v_ref_internal = self.step.add_computed_velocity(ComputedVelocityInternal {
            v,
            sampling_position: self.sref,
            contributions: self.contributions,
        });
        self.step.last_computed_velocity = Some(v_ref_internal);
        VelocityRef::new(v_ref_internal, self.step)
    }
}

impl PositionContributionInternal {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::StartPosition { .. } => PhysicalQuantityKind::Position,
            Self::VelocityDt { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDtDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    fn evaluate_for(&self, step: &IntegrationStep) -> Position {
        match self {
            Self::StartPosition { sref } => step.internal_get_position(*sref).s,
            Self::VelocityDt {
                factor,
                vref,
                dt_fraction,
            } => {
                let v = step.internal_get_velocity(*vref);
                factor * v * dt_fraction * step.dt
            }
            Self::AccelerationDtDt {
                factor,
                aref,
                dt_fraction,
            } => {
                let a = step.internal_get_acceleration(*aref);
                factor * a * (dt_fraction * step.dt) * (dt_fraction * step.dt)
            }
        }
    }
}

impl VelocityContributionInternal {
    fn kind(&self) -> PhysicalQuantityKind {
        match self {
            Self::Velocity { .. } => PhysicalQuantityKind::Velocity,
            Self::AccelerationDt { .. } => PhysicalQuantityKind::Acceleration,
        }
    }

    fn evaluate_for(&self, step: &IntegrationStep) -> Velocity {
        match self {
            Self::Velocity { vref } => step.internal_get_velocity(*vref).v,
            Self::AccelerationDt {
                factor,
                aref,
                dt_fraction,
            } => {
                let a = step.internal_get_acceleration(*aref);
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
