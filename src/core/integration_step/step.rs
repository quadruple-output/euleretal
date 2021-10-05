use super::{
    builders,
    core::{
        integration_step::{computed, contributions, StartCondition},
        integrator, Acceleration, AccelerationField, Duration, Fraction, Position, Velocity,
    },
    import::{shape, PointQuery},
};

pub struct Step {
    dt: Duration,
    positions: Vec<computed::Position>,
    velocities: Vec<computed::Velocity>,
    accelerations: Vec<computed::Acceleration>,
    last_computed_position: Option<PositionRef>,
    last_computed_velocity: Option<VelocityRef>,
    acceleration_at_last_position: Option<AccelerationRef>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PositionRef(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VelocityRef(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AccelerationRef(usize);

#[derive(Clone, Copy, Debug)]
pub struct ConditionRef {
    pub s: PositionRef,
    pub v: VelocityRef,
    pub a: AccelerationRef,
}

impl Step {
    pub fn new_deprecated(capacities: integrator::ExpectedCapacities, dt: Duration) -> Self {
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

    pub fn new(dt: Duration) -> Self {
        Self {
            dt,
            positions: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            last_computed_position: None,
            last_computed_velocity: None,
            acceleration_at_last_position: None,
        }
    }

    pub fn new_next(&self) -> Self {
        let mut next = Self {
            dt: self.dt,
            positions: Vec::with_capacity(self.positions.len()),
            velocities: Vec::with_capacity(self.velocities.len()),
            accelerations: Vec::with_capacity(self.accelerations.len()),
            last_computed_position: None,
            last_computed_velocity: None,
            acceleration_at_last_position: None,
        };
        next.set_start_condition(&self.next_condition().unwrap());
        next
    }

    pub fn raw_end_condition(&mut self, s: Position, v: Velocity, a: Acceleration) {
        let p_ref = self.add_computed_position(
            s,
            fraction!(1 / 1),
            contributions::position::Collection::empty(),
        );
        self.add_computed_velocity(v, p_ref, contributions::velocity::Collection::empty());
        self.acceleration_at_last_position = Some(self.add_computed_acceleration(a, p_ref));
    }

    pub fn set_start_condition(&mut self, p: &StartCondition) -> ConditionRef {
        let sref = self.add_computed_position(
            p.position(),
            fraction!(0 / 1),
            contributions::position::Collection::empty(),
        );
        ConditionRef {
            s: sref,
            v: self.add_computed_velocity(
                p.velocity(),
                sref,
                contributions::velocity::Collection::empty(),
            ),
            a: self.add_computed_acceleration(p.acceleration(), sref),
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
        self.add_computed_acceleration(a.value_at(self[sref].s), sref)
    }

    pub fn compute_acceleration_at_last_position(&mut self, a: &dyn AccelerationField) {
        let last_pref = self.last_computed_position.unwrap();
        self.acceleration_at_last_position =
            Some(self.add_computed_acceleration(a.value_at(self[last_pref].s), last_pref));
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn last_computed_position(&self) -> computed::position::Abstraction {
        self[self.last_computed_position.unwrap()].abstraction_for(self)
    }

    pub fn last_computed_velocity(&self) -> computed::velocity::Abstraction {
        self[self.last_computed_velocity.unwrap()].abstraction_for(self)
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

    pub fn closest_computed_velocity(
        &self,
        pos: impl Into<Position>,
    ) -> computed::velocity::Abstraction {
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
            .abstraction_for(self)
    }

    pub fn closest_computed_position(
        &self,
        pos: impl Into<Position>,
    ) -> computed::position::Abstraction {
        let pos = pos.into();
        self.positions
            .iter()
            .filter(|p| !p.contributions.0.is_empty()) // no predecessor → not 'computed'
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
            .abstraction_for(self)
    }

    pub(super) fn last_position_ref(&self) -> PositionRef {
        PositionRef(self.positions.len() - 1)
    }

    pub(super) fn add_computed_position(
        &mut self,
        s: Position,
        dt_fraction: Fraction,
        contributions: contributions::position::Collection,
    ) -> PositionRef {
        let p_ref = PositionRef(self.positions.len());
        self.positions.push(computed::Position {
            s,
            dt_fraction,
            contributions,
        });
        self.last_computed_position = Some(p_ref);
        p_ref
    }

    pub(super) fn add_computed_velocity(
        &mut self,
        v: Velocity,
        sampling_position: PositionRef,
        contributions: contributions::velocity::Collection,
    ) -> VelocityRef {
        let v_ref = VelocityRef(self.velocities.len());
        self.velocities.push(computed::Velocity {
            v,
            sampling_position,
            contributions,
        });
        self.last_computed_velocity = Some(v_ref);
        v_ref
    }

    pub(super) fn add_computed_acceleration(
        &mut self,
        a: Acceleration,
        sampling_position: PositionRef,
    ) -> AccelerationRef {
        let a_ref = AccelerationRef(self.accelerations.len());
        self.accelerations.push(computed::Acceleration {
            a,
            sampling_position,
        });
        a_ref
    }

    pub fn get_start_condition(&self) -> StartCondition {
        StartCondition::new(
            self.positions[0].s,
            self.velocities[0].v,
            self.accelerations[0].a,
        )
    }
}

impl ::std::ops::Index<AccelerationRef> for Step {
    type Output = computed::Acceleration;

    fn index(&self, a_ref: AccelerationRef) -> &Self::Output {
        &self.accelerations[a_ref.0]
    }
}

impl ::std::ops::Index<PositionRef> for Step {
    type Output = computed::Position;

    fn index(&self, p_ref: PositionRef) -> &Self::Output {
        &self.positions[p_ref.0]
    }
}

impl ::std::ops::Index<VelocityRef> for Step {
    type Output = computed::Velocity;

    fn index(&self, v_ref: VelocityRef) -> &Self::Output {
        &self.velocities[v_ref.0]
    }
}
