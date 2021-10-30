use super::{
    computed, contributions,
    contributions::DtFraction,
    import::{shape, PointQuery},
    StartCondition,
};
use crate::{Acceleration, AccelerationField, Duration, Position, Velocity};

#[derive(Clone)]
pub struct Step {
    dt: Duration,
    positions: Vec<computed::Position>,
    velocities: Vec<computed::Velocity>,
    accelerations: Vec<computed::Acceleration>,
    last_computed_position: PositionRef,
    last_computed_velocity: VelocityRef,
    acceleration_at_last_position: AccelerationRef,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PositionRef(usize);

impl<const N: usize, const D: usize>
    ::std::ops::Add<contributions::position::Variant<DtFraction<N, D>>> for PositionRef
{
    type Output = contributions::position::Collection<N, D>;

    fn add(self, rhs: contributions::position::Variant<DtFraction<N, D>>) -> Self::Output {
        vec![self.into(), rhs].into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VelocityRef(usize);

impl<const N: usize, const D: usize> ::std::ops::Mul<DtFraction<N, D>> for VelocityRef {
    type Output = contributions::position::Variant<DtFraction<N, D>>;

    fn mul(self, rhs: DtFraction<N, D>) -> Self::Output {
        contributions::position::Variant::VelocityDt {
            factor: 1.,
            v_ref: self,
            dt_fraction: rhs,
        }
    }
}

impl<const N: usize, const D: usize>
    ::std::ops::Add<contributions::velocity::Variant<DtFraction<N, D>>> for VelocityRef
{
    type Output = contributions::velocity::Collection<N, D>;

    fn add(self, rhs: contributions::velocity::Variant<DtFraction<N, D>>) -> Self::Output {
        vec![self.into(), rhs].into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AccelerationRef(usize);

impl<const N: usize, const D: usize> ::std::ops::Mul<DtFraction<N, D>> for AccelerationRef {
    type Output = contributions::velocity::Variant<DtFraction<N, D>>;

    fn mul(self, rhs: DtFraction<N, D>) -> Self::Output {
        contributions::velocity::Variant::AccelerationDt {
            factor: 1.,
            a_ref: self,
            dt_fraction: rhs,
        }
    }
}

impl ::std::ops::Mul<AccelerationRef> for f32 {
    type Output = contributions::acceleration::Variant;

    fn mul(self, a_ref: AccelerationRef) -> Self::Output {
        contributions::acceleration::Variant::Acceleration {
            factor: self,
            a_ref,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ConditionRef {
    pub s: PositionRef,
    pub v: VelocityRef,
    pub a: AccelerationRef,
}

impl Step {
    #[must_use]
    pub fn new(start_condition: &StartCondition, dt: Duration) -> Self {
        let mut result = Self {
            dt,
            positions: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            last_computed_position: PositionRef::default(),
            last_computed_velocity: VelocityRef::default(),
            acceleration_at_last_position: AccelerationRef::default(),
        };
        result.set_start_condition(start_condition);
        result
    }

    #[must_use]
    pub fn create_next(&self) -> Self {
        let mut next = Self {
            dt: self.dt,
            positions: Vec::with_capacity(self.positions.capacity()),
            velocities: Vec::with_capacity(self.velocities.capacity()),
            accelerations: Vec::with_capacity(self.accelerations.capacity()),
            last_computed_position: PositionRef::default(),
            last_computed_velocity: VelocityRef::default(),
            acceleration_at_last_position: AccelerationRef::default(),
        };
        next.set_start_condition(&self.next_condition());
        next
    }

    pub fn raw_end_condition(&mut self, s: Position, v: Velocity, a: Acceleration) {
        let p_ref = self.add_computed_position(
            s,
            DtFraction::<1, 1>,
            contributions::position::Collection::empty(),
        );
        self.add_computed_velocity(
            v,
            p_ref,
            DtFraction::<1, 1>,
            contributions::velocity::Collection::empty(),
        );
        self.acceleration_at_last_position = self.add_computed_acceleration(a, p_ref);
    }

    pub fn set_start_condition(&mut self, p: &StartCondition) -> ConditionRef {
        let sref = self.add_computed_position(
            p.position(),
            DtFraction::<0, 1>,
            contributions::position::Collection::empty(),
        );
        ConditionRef {
            s: sref,
            v: self.add_computed_velocity(
                p.velocity(),
                sref,
                DtFraction::<0, 1>,
                contributions::velocity::Collection::empty(),
            ),
            a: self.add_computed_acceleration(p.acceleration(), sref),
        }
    }

    #[must_use]
    pub fn next_condition(&self) -> StartCondition {
        StartCondition::new(
            self[self.last_computed_position].s,
            self[self.last_computed_velocity].v,
            self[self.acceleration_at_last_position].a,
        )
    }

    pub fn compute_acceleration_at_last_position(&mut self, a: &dyn AccelerationField) {
        let last_pref = self.last_computed_position;
        self.acceleration_at_last_position =
            self.add_computed_acceleration(a.value_at(self[last_pref].s), last_pref);
    }

    #[must_use]
    pub fn dt(&self) -> Duration {
        self.dt
    }

    #[must_use]
    pub fn last_computed_position(&self) -> computed::position::Abstraction {
        self[self.last_computed_position].abstraction_for(self)
    }

    #[must_use]
    pub fn last_computed_velocity(&self) -> computed::velocity::Abstraction {
        self[self.last_computed_velocity].abstraction_for(self)
    }

    #[must_use]
    pub fn last_s(&self) -> Position {
        self[self.last_computed_position].s
    }

    #[must_use]
    pub fn last_v(&self) -> Velocity {
        self[self.last_computed_velocity].v
    }

    pub fn positions_iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.positions.iter().map(|comp_pos| comp_pos.s)
    }

    #[allow(clippy::missing_panics_doc)] // not expected to panic because the iterator will not be empty
    #[must_use]
    pub fn distance_to(&self, pos: &Position) -> f32 {
        shape::Segment::new(
            self.positions_iter().next().unwrap().into(),
            self.positions_iter().last().unwrap().into(), // next() can be same as last()
        )
        .distance_to_local_point(pos.as_point(), true)
    }

    #[allow(clippy::missing_panics_doc)] // not expected to panic because the iterator will not be empty
    #[must_use]
    pub fn closest_computed_velocity(
        &self,
        pos: impl Into<Position>,
    ) -> computed::velocity::Abstraction {
        let pos = pos.into();
        self.velocities
            .iter()
            .filter(|v| v.has_contributions()) // no predecessor → not 'computed'
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

    #[allow(clippy::missing_panics_doc)] // not expected to panic because the iterator will not be empty
    #[must_use]
    pub fn closest_computed_position(
        &self,
        pos: impl Into<Position>,
    ) -> computed::position::Abstraction {
        let pos = pos.into();
        self.positions
            .iter()
            .filter(|p| p.has_contributions()) // no predecessor → not 'computed'
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

    pub(super) fn last_velocity_ref(&self) -> VelocityRef {
        VelocityRef(self.velocities.len() - 1)
    }

    /// parameter DtFraction<N,D> improves readability at calling positions
    pub(super) fn add_computed_position<const N: usize, const D: usize>(
        &mut self,
        s: Position,
        _dt_fraction: DtFraction<N, D>,
        contributions: contributions::position::Collection<N, D>,
    ) -> PositionRef {
        let p_ref = PositionRef(self.positions.len());
        self.positions
            .push(computed::Position::new(s, contributions));
        self.last_computed_position = p_ref;
        p_ref
    }

    /// parameter DtFraction<N,D> improves readability at calling positions
    pub(super) fn add_computed_velocity<const N: usize, const D: usize>(
        &mut self,
        v: Velocity,
        sampling_position: PositionRef,
        _dt_fraction: DtFraction<N, D>,
        contributions: contributions::velocity::Collection<N, D>,
    ) -> VelocityRef {
        let v_ref = VelocityRef(self.velocities.len());
        self.velocities
            .push(computed::Velocity::new(v, sampling_position, contributions));
        self.last_computed_velocity = v_ref;
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

    #[must_use]
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

impl ::std::ops::IndexMut<AccelerationRef> for Step {
    fn index_mut(&mut self, a_ref: AccelerationRef) -> &mut Self::Output {
        &mut self.accelerations[a_ref.0]
    }
}

impl ::std::ops::Index<PositionRef> for Step {
    type Output = computed::Position;

    fn index(&self, p_ref: PositionRef) -> &Self::Output {
        &self.positions[p_ref.0]
    }
}

impl ::std::ops::IndexMut<PositionRef> for Step {
    fn index_mut(&mut self, p_ref: PositionRef) -> &mut Self::Output {
        &mut self.positions[p_ref.0]
    }
}

impl ::std::ops::Index<VelocityRef> for Step {
    type Output = computed::Velocity;

    fn index(&self, v_ref: VelocityRef) -> &Self::Output {
        &self.velocities[v_ref.0]
    }
}

impl ::std::ops::IndexMut<VelocityRef> for Step {
    fn index_mut(&mut self, v_ref: VelocityRef) -> &mut Self::Output {
        &mut self.velocities[v_ref.0]
    }
}
