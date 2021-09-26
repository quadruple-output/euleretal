#![cfg(test)]

use super::core::{self, integration_step::builders, AccelerationField};
use super::scenarios;
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

trait Integrator: Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate_step(&self, builder: &builders::Step, acceleration_field: &dyn AccelerationField);

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }

    /// Number of acceleration values involved in computing the next sample. This does not include
    /// the acceleration value at the computed next sample.
    fn expected_accelerations_for_step(&self) -> usize;

    /// Number of positions involved in computing the next sample. This doen not include the
    /// position of the next sample.
    fn expected_positions_for_step(&self) -> usize;

    /// Number of velocity values involved in computing the next sample. This does not include the
    /// computed velocity of the next sample.
    fn expected_velocities_for_step(&self) -> usize;

    fn expected_capacities_for_step(&self) -> ExpectedCapacities {
        ExpectedCapacities {
            _positions: self.expected_positions_for_step(),
            _velocities: self.expected_velocities_for_step(),
            _accelerations: self.expected_accelerations_for_step(),
        }
    }
}

#[derive(Clone, Copy)]
struct ExpectedCapacities {
    _positions: usize,
    _velocities: usize,
    _accelerations: usize,
}

impl Default for ExpectedCapacities {
    fn default() -> Self {
        Self {
            _positions: 1,
            _velocities: 1,
            _accelerations: 1,
        }
    }
}

struct TheIntegrator;

impl TheIntegrator {
    const fn new() -> Self {
        Self
    }
}

impl Integrator for TheIntegrator {
    fn label(&self) -> String {
        "Test dummy".to_string()
    }

    fn description(&self) -> String {
        "Test dummy".to_string()
    }

    fn expected_accelerations_for_step(&self) -> usize {
        1
    }

    fn expected_positions_for_step(&self) -> usize {
        1
    }

    fn expected_velocities_for_step(&self) -> usize {
        1
    }

    /// This is an experimental example of how an integration step might ideally be implemented.
    fn integrate_step(&self, builder: &builders::Step, _a: &dyn AccelerationField) {
        let (_s0, _v0, _a0) = builder.start_values();
        let _dt = builder.dt();
        //let v1 = v0 + a0 * dt;
        //let s1 = s0 + v1 * dt;
    }
}

mod tests {
    use super::{
        core::{
            integration_step::{builders, StartCondition, Step},
            AccelerationField, Duration, Position, Velocity,
        },
        scenarios, Integrator, TheIntegrator,
    };

    fn calc_step(start_position: Position, start_velocity: Velocity, dt: Duration) -> Step {
        let acceleration_field = scenarios::ConstantAcceleration;
        let start = StartCondition::new(
            start_position,
            start_velocity,
            acceleration_field.value_at(start_position),
        );
        let builder = builders::Step::new(&acceleration_field, &start, dt);
        TheIntegrator::new().integrate_step(&builder, &acceleration_field);
        builder.result()
    }

    #[test]
    fn resting_at_origin() {
        let _step = calc_step(Position::origin(), Velocity::zeros(), 0.5.into());
    }
}
