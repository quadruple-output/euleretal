use super::{
    derived_quantities::{DerivedPosition, DerivedVelocity},
    import::R32,
    Acceleration, Position, Velocity,
};
use ::std::marker::PhantomData;

mod type_state {
    pub trait TypeState {}

    pub struct Finalized {}
    pub struct NonFinalized {}

    impl TypeState for Finalized {}
    impl TypeState for NonFinalized {}
}

use type_state::{Finalized, NonFinalized, TypeState};

pub struct Samples<TS: TypeState = Finalized> {
    steps: Vec<Step>,
    type_state: PhantomData<TS>,
}

pub struct Step {
    pub time: R32,
    pub dt: R32,
    pub derived_position: DerivedPosition,
    pub derived_velocity: DerivedVelocity,
    pub acceleration: Acceleration,
}

/// This implements the `new` method for `Samples<Finalized>`, returning `Samples<NonFinalized>`.
/// This seems odd, but it is convenient since `Finalized` is the default type.
impl Samples {
    pub fn new(start_condition: &StartCondition, sample_capacity: usize) -> Samples<NonFinalized> {
        let mut instance = Samples::<NonFinalized> {
            steps: Vec::with_capacity(sample_capacity + 1), // +1 for the Endpoint
            type_state: PhantomData::<NonFinalized>,
        };
        instance.steps.push(Step {
            time: 0.0.into(), // start time is always zero
            dt: 0.0.into(),   // initial sample has no delta
            derived_position: start_condition.position.into(),
            derived_velocity: start_condition.velocity.into(),
            acceleration: start_condition.acceleration,
        });
        instance
    }
}

impl Samples<NonFinalized> {
    pub fn push_sample(&mut self, sample: NewSampleWithPoints) {
        self.steps.push(Step {
            time: self.steps.last().unwrap().time + sample.dt,
            dt: sample.dt,
            derived_position: sample.position,
            derived_velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
    }

    #[must_use]
    pub fn current(&self) -> Option<StartCondition> {
        self.steps.last().map(|current_step| StartCondition {
            position: (&current_step.derived_position).into(),
            velocity: (&current_step.derived_velocity).into(),
            acceleration: current_step.acceleration,
        })
    }

    #[must_use]
    pub fn finalized(self) -> Samples<Finalized> {
        Samples {
            steps: self.steps,
            type_state: PhantomData::<Finalized>,
        }
    }
}

impl Samples<Finalized> {
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn step_positions(&self) -> PositionIter {
        PositionIter {
            steps_iter: self.steps.iter(),
        }
    }

    pub fn at(&self, idx: usize) -> &Step {
        &self.steps[idx]
    }
}

pub struct PositionIter<'a> {
    steps_iter: ::std::slice::Iter<'a, Step>,
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.steps_iter
            .next()
            .map(|step| (&step.derived_position).into())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.steps_iter.size_hint()
    }

    fn count(self) -> usize {
        self.steps_iter.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.steps_iter
            .last()
            .map(|step| (&step.derived_position).into())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.steps_iter
            .nth(n)
            .map(|step| (&step.derived_position).into())
    }
}

#[derive(Clone)]
pub struct StartCondition {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Default)]
pub struct NewSampleWithPoints {
    pub dt: R32,
    pub position: DerivedPosition,
    pub velocity: DerivedVelocity,
    pub acceleration: Acceleration,
}
