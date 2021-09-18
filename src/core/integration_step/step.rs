use super::{
    builders,
    core::{
        integration_step::{
            ComputedAcceleration, ComputedPosition, ComputedPositionData, ComputedVelocity,
            ComputedVelocityData, StartCondition,
        },
        integrator, Acceleration, AccelerationField, Duration, Fraction, Position, Velocity,
    },
    import::{shape, PointQuery},
};

pub struct IntegrationStep {
    dt: Duration,
    positions: Vec<ComputedPositionData>,
    velocities: Vec<ComputedVelocityData>,
    accelerations: Vec<ComputedAcceleration>,
    last_computed_position: Option<PositionRef>,
    last_computed_velocity: Option<VelocityRef>,
    acceleration_at_last_position: Option<AccelerationRef>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PositionRef(usize);

#[derive(Clone, Copy, PartialEq)]
pub struct VelocityRef(usize);

#[derive(Clone, Copy, PartialEq)]
pub struct AccelerationRef(usize);

#[derive(Clone, Copy)]
pub struct ConditionRef {
    pub s: PositionRef,
    pub v: VelocityRef,
    pub a: AccelerationRef,
}

impl IntegrationStep {
    pub fn new(capacities: integrator::ExpectedCapacities, dt: Duration) -> Self {
        Self {
            dt,
            positions: Vec::with_capacity(capacities.positions + 1),
            velocities: Vec::with_capacity(capacities.velocities + 1),
            accelerations: Vec::with_capacity(capacities.accelerations + 1),
            last_computed_position: None,
            last_computed_velocity: None,
            acceleration_at_last_position: None,
        }
    }

    pub fn raw_end_condition(&mut self, s: Position, v: Velocity, a: Acceleration) {
        let p_ref = self.add_computed_position(ComputedPositionData {
            s,
            dt_fraction: fraction!(1 / 1),
            contributions: Vec::new(),
        });
        self.last_computed_position = Some(p_ref);
        self.last_computed_velocity = Some(self.add_computed_velocity(ComputedVelocityData {
            v,
            sampling_position: p_ref,
            contributions: Vec::new(),
        }));
        self.acceleration_at_last_position =
            Some(self.add_computed_acceleration(ComputedAcceleration {
                a,
                sampling_position: p_ref,
            }));
    }

    pub fn start_position(&mut self, s: Position) -> PositionRef {
        self.add_computed_position(ComputedPositionData {
            s,
            dt_fraction: fraction!(0 / 1),
            contributions: Vec::new(),
        })
    }

    pub fn start_velocity(&mut self, v: Velocity, sampling_position: PositionRef) -> VelocityRef {
        self.add_computed_velocity(ComputedVelocityData {
            v,
            sampling_position,
            contributions: Vec::new(),
        })
    }

    pub fn start_acceleration(
        &mut self,
        a: Acceleration,
        sampling_position: PositionRef,
    ) -> AccelerationRef {
        self.add_computed_acceleration(ComputedAcceleration {
            a,
            sampling_position,
        })
    }

    pub fn initial_condition(&mut self, p: &StartCondition) -> ConditionRef {
        let sref = self.start_position(p.position());
        ConditionRef {
            s: sref,
            v: self.start_velocity(p.velocity(), sref),
            a: self.start_acceleration(p.acceleration(), sref),
        }
    }

    pub fn next_condition(&self) -> Option<StartCondition> {
        if let (Some(p_ref), Some(v_ref), Some(a_ref)) = (
            self.last_computed_position,
            self.last_computed_velocity,
            self.acceleration_at_last_position,
        ) {
            Some(StartCondition::new(
                self[p_ref].s,
                self[v_ref].v,
                self[a_ref].a,
            ))
        } else {
            None
        }
    }

    pub fn compute_position(&mut self, dt_fraction: Fraction) -> builders::Position {
        builders::Position::new(self, dt_fraction)
    }

    pub fn compute_velocity(
        &mut self,
        dt_fraction: Fraction,
        sref: PositionRef,
    ) -> builders::Velocity {
        builders::Velocity::new(self, dt_fraction, sref)
    }

    pub fn compute_acceleration_at(
        &mut self,
        sref: PositionRef,
        a: &dyn AccelerationField,
    ) -> AccelerationRef {
        self.add_computed_acceleration(ComputedAcceleration {
            a: a.value_at(self[sref].s),
            sampling_position: sref,
        })
    }

    pub fn compute_acceleration_at_last_position(&mut self, a: &dyn AccelerationField) {
        let last_pref = self.last_computed_position.unwrap();
        self.acceleration_at_last_position =
            Some(self.add_computed_acceleration(ComputedAcceleration {
                a: a.value_at(self[last_pref].s),
                sampling_position: last_pref,
            }));
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn last_computed_position(&self) -> ComputedPosition {
        self[self.last_computed_position.unwrap()].public_for(self)
    }

    pub fn last_computed_velocity(&self) -> ComputedVelocity {
        self[self.last_computed_velocity.unwrap()].public_for(self)
    }

    pub fn last_s(&self) -> Position {
        self[self.last_computed_position.unwrap()].s
    }

    pub fn last_v(&self) -> Velocity {
        self[self.last_computed_velocity.unwrap()].v
    }

    pub fn positions_iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.positions.iter().map(|comp_pos| comp_pos.s)
    }

    pub fn distance_to(&self, pos: &Position) -> f32 {
        shape::Segment::new(
            self.positions_iter().next().unwrap().into(),
            self.positions_iter().last().unwrap().into(),
        )
        .distance_to_local_point(pos.as_point(), true)
    }

    pub fn closest_computed_velocity(&self, pos: impl Into<Position>) -> ComputedVelocity {
        let pos = pos.into();
        self.velocities
            .iter()
            .filter(|v| !v.contributions.is_empty()) // no predecessor → not 'computed'
            .map(|v| (v, self[v.sampling_position].s.distance_squared(pos)))
            .reduce(|(v1, dist1), (v2, dist2)| {
                if dist1 < dist2 {
                    (v1, dist1)
                } else {
                    (v2, dist2)
                }
            })
            .unwrap()
            .0
            .public_for(self)
    }

    pub fn closest_computed_position(&self, pos: impl Into<Position>) -> ComputedPosition {
        let pos = pos.into();
        self.positions
            .iter()
            .filter(|p| !p.contributions.is_empty()) // no predecessor → not 'computed'
            .map(|p| (p, p.s.distance_squared(pos)))
            .reduce(|(p1, dist1), (p2, dist2)| {
                if dist1 < dist2 {
                    (p1, dist1)
                } else {
                    (p2, dist2)
                }
            })
            .unwrap()
            .0
            .public_for(self)
    }

    pub(super) fn add_computed_position(&mut self, p: ComputedPositionData) -> PositionRef {
        self.positions.push(p);
        let p_ref = PositionRef(self.positions.len() - 1);
        self.last_computed_position = Some(p_ref);
        p_ref
    }

    pub(super) fn add_computed_velocity(&mut self, p: ComputedVelocityData) -> VelocityRef {
        self.velocities.push(p);
        let v_ref = VelocityRef(self.velocities.len() - 1);
        self.last_computed_velocity = Some(v_ref);
        v_ref
    }

    pub(super) fn add_computed_acceleration(&mut self, p: ComputedAcceleration) -> AccelerationRef {
        self.accelerations.push(p);
        AccelerationRef(self.accelerations.len() - 1)
    }

    pub fn get_start_condition(&self) -> StartCondition {
        StartCondition::new(
            self.positions[0].s,
            self.velocities[0].v,
            self.accelerations[0].a,
        )
    }
}

impl ::std::ops::Index<AccelerationRef> for IntegrationStep {
    type Output = ComputedAcceleration;

    fn index(&self, a_ref: AccelerationRef) -> &Self::Output {
        &self.accelerations[a_ref.0]
    }
}

impl ::std::ops::Index<PositionRef> for IntegrationStep {
    type Output = ComputedPositionData;

    fn index(&self, p_ref: PositionRef) -> &Self::Output {
        &self.positions[p_ref.0]
    }
}

impl ::std::ops::Index<VelocityRef> for IntegrationStep {
    type Output = ComputedVelocityData;

    fn index(&self, v_ref: VelocityRef) -> &Self::Output {
        &self.velocities[v_ref.0]
    }
}
