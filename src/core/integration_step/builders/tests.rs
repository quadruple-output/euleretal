#![cfg(test)]

use super::integration_step::StartCondition;
use super::{
    core::{AccelerationField, Duration, Position, Step, Velocity},
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

fn create_builder(ctx: &Setup) -> StepBuilder {
    StepBuilder::new(&ctx.start_condition, ctx.dt)
}

#[test]
fn create_step_builder() {
    let ctx = Setup::default();
    let builder = create_builder(&ctx);
    let step = builder.result();
    assert_eq!(step.get_start_condition(), ctx.start_condition);
}
