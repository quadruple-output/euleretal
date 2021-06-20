use super::{Acceleration, IntegrationStep, Position, Velocity};
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
    steps: Vec<IntegrationStep>,
    type_state: PhantomData<TS>,
}

/// This implements the `new` method for `Samples<Finalized>`, returning `Samples<NonFinalized>`.
/// This seems odd, but it is convenient since `Finalized` is the default type.
impl Samples {
    pub fn new(sample_capacity: usize) -> Samples<NonFinalized> {
        Samples::<NonFinalized> {
            steps: Vec::with_capacity(sample_capacity),
            type_state: PhantomData::<NonFinalized>,
        }
    }
}

impl Samples<NonFinalized> {
    pub fn push_sample(&mut self, step: IntegrationStep) {
        self.steps.push(step);
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

    pub fn at(&self, idx: usize) -> &IntegrationStep {
        &self.steps[idx]
    }
}

pub struct PositionIter<'a> {
    steps_iter: ::std::slice::Iter<'a, IntegrationStep>,
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.steps_iter.next().map(Self::step_to_s)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.steps_iter.size_hint()
    }

    fn count(self) -> usize {
        self.steps_iter.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.steps_iter.last().map(Self::step_to_s)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.steps_iter.nth(n).map(Self::step_to_s)
    }
}

impl<'a> PositionIter<'a> {
    pub fn step_to_s(step: &IntegrationStep) -> Position {
        step.last_s()
    }
}

#[derive(Clone)]
pub struct StartCondition {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}
