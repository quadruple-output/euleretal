#![cfg(test)]

use super::integration_step::StartCondition;
use super::{
    core::{Acceleration, AccelerationField, Duration, Position, Step, Velocity},
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
    fn new_builder_for<'a>(&'a self, step: &'a mut Step) -> StepBuilder<'a> {
        StepBuilder::new(&self.acceleration_field, &self.start_condition, step)
    }

    fn new_step(&self) -> Step {
        Step::new(self.dt)
    }

    fn start_values(&self) -> (Position, Velocity, Acceleration) {
        (
            self.start_condition.position(),
            self.start_condition.velocity(),
            self.start_condition.acceleration(),
        )
    }
}

#[test]
fn step_from_new_builder_has_correct_start_condition() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);
    builder.finalize();
    assert_eq!(step.get_start_condition(), ctx.start_condition);
}

#[test]
fn create_step_from_previous() {
    let ctx = Setup::default();
    let mut step0 = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step0);
    builder.finalize();
    let mut step1 = builder.next_step();
    builder.next_for(&mut step1).finalize();
    assert_eq!(step0.dt(), step1.dt());
    assert_eq!(step0.get_start_condition(), step1.get_start_condition());
}

#[test]
fn trivial_step_with_p1_eq_p0() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    let (s0, v0, _a0) = builder.start_values();
    builder.push(s0);
    builder.push(v0);

    builder.finalize();
    {
        let computed_position = step.last_computed_position();
        assert_eq!(computed_position.s(), ctx.start_condition.position());
        assert_eq!(computed_position.dt_fraction(), fraction!(0 / 1));
        let mut position_contribs = computed_position.contributions_iter();
        let position_contribution = position_contribs.next().unwrap();
        assert_eq!(
            position_contribution.sampling_position(),
            ctx.start_condition.position()
        );
        assert!(position_contribs.next().is_none());
    }

    {
        let computed_velocity = step.last_computed_velocity();
        assert_eq!(computed_velocity.v(), ctx.start_condition.velocity());
        assert_eq!(
            computed_velocity.sampling_position(),
            ctx.start_condition.position()
        );
        let mut velocity_contribs = computed_velocity.contributions_iter();
        let velocity_contribution = velocity_contribs.next().unwrap();
        assert_eq!(
            velocity_contribution.vector(),
            ctx.start_condition.velocity()
        );
        assert_eq!(
            velocity_contribution.sampling_position(),
            ctx.start_condition.position()
        );
        assert!(velocity_contribs.next().is_none());
    }
    {
        let next_condition = step.next_condition().unwrap();
        assert_eq!(next_condition, ctx.start_condition);
    }
}

#[test]
fn simple_step_s_v_dt() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);
    // calculate step:
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    builder.finalize();

    // expected results:
    let ((s0, v0, _a0), dt) = (ctx.start_values(), ctx.dt);
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

    // calculate two steps:
    let mut step1 = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step1);
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    builder.finalize();
    let mut step2 = builder.next_step();
    let mut builder = builder.next_for(&mut step2);
    {
        let (s0, v0, _a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt);
    }
    builder.finalize();

    // expected result:
    let ((s0, v0, _a0), dt) = (ctx.start_values(), ctx.dt);
    let s1 = s0 + v0 * dt;
    let v1 = v0;
    let s2 = s1 + v1 * dt;

    // check result:
    let final_position = step2.last_computed_position();
    assert_eq!(final_position.s(), s2);

    let mut contributions = final_position.contributions_iter();
    let first_contribution = contributions.next().unwrap();
    let second_contribution = contributions.next().unwrap();
    assert!(contributions.next().is_none());

    assert_eq!(first_contribution.sampling_position(), s1);
    assert_eq!(second_contribution.sampling_position(), s1);
    assert_eq!(second_contribution.vector().unwrap(), v1 * dt);
}

#[test]
fn simple_step_s_a_dt_dt() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    {
        let (s0, _v0, a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + a0 * dt * dt);
        builder.finalize();
    }

    let ((s0, _v0, a0), dt) = (ctx.start_values(), ctx.dt);

    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s0 + a0 * dt * dt);

    let mut contributions = final_position.contributions_iter();
    let first_contribution = contributions.next().unwrap();
    let second_contribution = contributions.next().unwrap();
    assert!(contributions.next().is_none());

    assert_eq!(first_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.vector().unwrap(), a0 * dt * dt);
}

#[test]
fn simple_step_s_v_dt_12_a_dt_dt() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    {
        let (s0, v0, a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(s0 + v0 * dt + 0.5 * a0 * dt * dt);
        builder.finalize();
    }

    let ((s0, v0, a0), dt) = (ctx.start_values(), ctx.dt);

    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s0 + v0 * dt + 0.5 * a0 * dt * dt);

    let mut contributions = final_position.contributions_iter();
    let first_contribution = contributions.next().unwrap();
    let second_contribution = contributions.next().unwrap();
    let third_contribution = contributions.next().unwrap();
    assert!(contributions.next().is_none());

    assert_eq!(first_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.sampling_position(), s0);
    assert_eq!(third_contribution.sampling_position(), s0);
    assert_eq!(second_contribution.vector().unwrap(), v0 * dt);
    assert_eq!(third_contribution.vector().unwrap(), 0.5 * a0 * dt * dt);
}

#[test]
fn euler() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    // test:
    {
        let (s0, v0, a0) = builder.start_values();
        let dt = builder.dt();
        builder.push(v0 + a0 * dt);
        builder.push(s0 + v0 * dt + a0 * dt * dt);
        builder.finalize();
    }

    // expected values:
    let ((s0, v0, a0), dt) = (ctx.start_values(), ctx.dt);
    let s1 = s0 + v0 * dt + a0 * dt * dt;
    let v1 = v0 + a0 * dt;

    // assertions:
    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s1);

    let final_velocity = step.last_computed_velocity();
    assert_eq!(final_velocity.v(), v1);
    // tested elsewhere:
    // assert_eq!(final_velocity.sampling_position(), s1);

    let mut s_contribs = final_position.contributions_iter();
    let s_contrib_1 = s_contribs.next().unwrap();
    let s_contrib_2 = s_contribs.next().unwrap();
    let s_contrib_3 = s_contribs.next().unwrap();
    assert!(s_contribs.next().is_none());

    assert_eq!(s_contrib_1.sampling_position(), s0);
    assert_eq!(s_contrib_2.sampling_position(), s0);
    assert_eq!(s_contrib_2.vector().unwrap(), v0 * dt);
    assert_eq!(s_contrib_3.sampling_position(), s0);
    assert_eq!(s_contrib_3.vector().unwrap(), a0 * dt * dt);

    let mut v_contribs = final_velocity.contributions_iter();
    let v_contrib_1 = v_contribs.next().unwrap();
    let v_contrib_2 = v_contribs.next().unwrap();
    assert!(v_contribs.next().is_none());

    assert_eq!(v_contrib_1.sampling_position(), s0);
    assert_eq!(v_contrib_1.vector(), v0);
    assert_eq!(v_contrib_2.sampling_position(), s0);
    assert_eq!(v_contrib_2.vector(), a0 * dt);
}

/*
#[test]
fn can_set_display_position_of_velocity() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    // test:
    {
        let (s0, v0, a0) = builder.start_values();
        let dt = builder.dt();
        let v1 = builder.push(v0 + a0 * dt);
        let s1 = builder.push(s0 + v0 * dt + 0.5 * a0 * dt * dt);
        v1.display_at(s1);
        builder.finalize();
    }

    // expected values:
    let ((s0, v0, a0), dt) = (ctx.start_values(), ctx.dt);
    let s1 = s0 + v0 * dt + 0.5 * a0 * dt * dt;
    let v1 = v0 + a0 * dt;

    // assertions:
    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s1);

    let final_velocity = step.last_computed_velocity();
    assert_eq!(final_velocity.v(), v1);
    assert_eq!(final_velocity.sampling_position(), s1);
}
 */

#[test]
fn euler_with_intermediate_v() {
    let ctx = Setup::default();
    let mut step = ctx.new_step();
    let mut builder = ctx.new_builder_for(&mut step);

    // test:
    {
        let ((s0, v0, a0), dt) = (builder.start_values(), builder.dt());
        let v1 = builder.push(v0 + a0 * dt);
        builder.push(s0 + v1 * dt);
    }

    // expected values:
    let ((s0, v0, a0), dt) = (ctx.start_values(), ctx.dt);
    let v1 = v0 + a0 * dt;
    let s1 = s0 + v1 * dt;

    // assertions:
    let final_position = step.last_computed_position();
    assert_eq!(final_position.s(), s1);

    let final_velocity = step.last_computed_velocity();
    assert_eq!(final_velocity.v(), v1);
    // tested elsewhere:
    // assert_eq!(final_velocity.sampling_position(), s1);

    let mut s_contribs = final_position.contributions_iter();
    let s_contrib_1 = s_contribs.next().unwrap();
    let s_contrib_2 = s_contribs.next().unwrap();
    assert!(s_contribs.next().is_none());

    assert_eq!(s_contrib_1.sampling_position(), s0);
    //assert_eq!(s_contrib_2.sampling_position(), s0);
    assert_eq!(s_contrib_2.vector().unwrap(), v1 * dt);

    let mut v_contribs = final_velocity.contributions_iter();
    let v_contrib_1 = v_contribs.next().unwrap();
    let v_contrib_2 = v_contribs.next().unwrap();
    assert!(v_contribs.next().is_none());

    assert_eq!(v_contrib_1.sampling_position(), s0);
    assert_eq!(v_contrib_1.vector(), v0);
    assert_eq!(v_contrib_2.sampling_position(), s0);
    assert_eq!(v_contrib_2.vector(), a0 * dt);
}
