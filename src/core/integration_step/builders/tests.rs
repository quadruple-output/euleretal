#![cfg(test)]

use super::integration_step::StartCondition;
use super::{
    core::{AccelerationField, Duration, Position, Velocity},
    integration_step::{builders::step::Push, step::PositionRef},
    Step as StepBuilder,
};
use crate::core::integrator::ExpectedCapacities;
// not used in super, so we use an absolute path (only for tests!):
use crate::scenarios;

struct Setup {
    acceleration_field: scenarios::CenterMass,
    start_condition: StartCondition,
    dt: Duration,
}

impl Default for Setup {
    fn default() -> Self {
        let acceleration_field = scenarios::CenterMass;
        let start_position = Position::new(1., 2., 3.);
        Self {
            acceleration_field,
            start_condition: StartCondition::new(
                start_position,
                Velocity::new(4., 5., 6.),
                acceleration_field.value_at(start_position),
            ),
            dt: 0.3.into(),
        }
    }
}

impl Setup {
    fn create_builder(&self) -> StepBuilder {
        StepBuilder::new(&self.start_condition, self.dt)
    }
}

#[test]
fn step_from_new_builder_has_correct_start_condition() {
    let ctx = Setup::default();
    let builder = ctx.create_builder();

    let step = builder.result();
    assert_eq!(step.get_start_condition(), ctx.start_condition);
}

#[test]
fn builder_returns_start_quantities() {
    let ctx = Setup::default();
    let builder = ctx.create_builder();

    let (s0, v0, a0) = builder.start_values();
    let step = builder.result();
    assert_eq!(
        ctx.start_condition.position(),
        step[PositionRef::from(s0)].s
    );
    assert_eq!(ctx.start_condition.velocity(), v0.into());
    assert_eq!(ctx.start_condition.acceleration(), a0.into());
}

#[test]
fn trivial_step_with_p1_eq_p0() {
    let ctx = Setup::default();
    let mut builder = ctx.create_builder();

    let (s0, _v0, _a0) = builder.start_values();
    builder.push(s0);

    let step = builder.result();
    let comp_pos = step.last_computed_position();
    assert_eq!(comp_pos.s(), ctx.start_condition.position());
    assert_eq!(comp_pos.dt_fraction(), fraction!(0 / 1));
    assert_eq!(
        comp_pos
            .contributions_iter()
            .next()
            .unwrap()
            .sampling_position(),
        ctx.start_condition.position()
    );
    assert!(comp_pos.contributions_iter().nth(1).is_none());
}
