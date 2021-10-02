#![cfg(test)]

use super::integration_step::StartCondition;
use super::{
    core::{AccelerationField, Duration, Position, Velocity},
    integration_step::builders::step::Push,
    Step as StepBuilder,
};
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
        StepBuilder::new(&self.acceleration_field, &self.start_condition, self.dt)
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
fn create_step_from_previous() {
    let ctx = Setup::default();
    let step0 = ctx.create_builder().result();
    let step1 = StepBuilder::from_previous(&ctx.acceleration_field, &step0).result();
    assert_eq!(step0.dt(), step1.dt());
    assert_eq!(step0.get_start_condition(), step1.get_start_condition());
}

#[test]
fn trivial_step_with_p1_eq_p0() {
    let ctx = Setup::default();
    let mut builder = ctx.create_builder();

    let (s0, v0, _a0) = builder.start_values();
    builder.push(s0);
    builder.push(v0);

    let step = builder.result();
    {
        let computed_position = step.last_computed_position();
        assert_eq!(computed_position.s(), ctx.start_condition.position());
        assert_eq!(computed_position.dt_fraction(), fraction!(0 / 1));
        assert_eq!(
            computed_position
                .contributions_iter()
                .next()
                .unwrap()
                .sampling_position(),
            ctx.start_condition.position()
        );
        assert!(computed_position.contributions_iter().nth(1).is_none());
    }

    {
        let computed_velocity = step.last_computed_velocity();
        assert_eq!(computed_velocity.v(), ctx.start_condition.velocity());
        assert_eq!(
            computed_velocity.sampling_position(),
            ctx.start_condition.position()
        );
        let velocity_contribution = computed_velocity.contributions_iter().next().unwrap();
        assert_eq!(
            velocity_contribution.vector(),
            ctx.start_condition.velocity()
        );
        assert_eq!(
            velocity_contribution.sampling_position(),
            ctx.start_condition.position()
        );
        assert!(computed_velocity.contributions_iter().nth(1).is_none());
    }
    {
        let next_condition = step.next_condition().unwrap();
        assert_eq!(next_condition, ctx.start_condition);
    }
}

#[test]
fn simple_step() {
    let ctx = Setup::default();
    let mut builder = ctx.create_builder();
    // calculate step:
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    let step = builder.result();

    // expected results:
    let (s0, v0, dt) = (
        ctx.start_condition.position(),
        ctx.start_condition.velocity(),
        ctx.dt,
    );
    let s1 = s0 + v0 * dt;

    // check calculated result:
    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s1);

    let mut contributions = final_position.contributions_iter();
    let first_contribution = contributions.next().unwrap();
    let second_contribution = contributions.next().unwrap();
    assert!(contributions.next().is_none());

    assert_eq!(first_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.vector().unwrap(), v0 * dt);
}

#[test]
fn two_simple_steps_in_sequence() {
    let ctx = Setup::default();
    let mut builder = ctx.create_builder();

    // calculate two steps:
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    let step = builder.result();

    let mut builder = StepBuilder::from_previous(&ctx.acceleration_field, &step);
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    let step = builder.result();

    // expected result:
    let s1;
    let v1;
    let dt = ctx.dt;
    {
        let (s0, v0) = (
            ctx.start_condition.position(),
            ctx.start_condition.velocity(),
        );
        s1 = s0 + v0 * dt;
        v1 = v0;
    }
    let s2 = s1 + v1 * dt;

    // check result:
    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s2);

    let mut contributions = final_position.contributions_iter();
    let first_contribution = contributions.next().unwrap();
    let second_contribution = contributions.next().unwrap();
    assert!(contributions.next().is_none());

    assert_eq!(first_contribution.sampling_position(), s1);
    assert_eq!(second_contribution.sampling_position(), s1);
    assert_eq!(second_contribution.vector().unwrap(), v1 * dt);
}
