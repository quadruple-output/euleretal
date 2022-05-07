mod type_state {
    pub trait TypeState {}

    pub struct Finalized {}
    pub struct NonFinalized {}

    impl TypeState for Finalized {}
    impl TypeState for NonFinalized {}
}

use super::{Position, Step};
use ::std::marker::PhantomData;
use type_state::{Finalized, NonFinalized, TypeState};

pub struct Samples<TS: TypeState = Finalized> {
    steps: Vec<Step>,
    type_state: PhantomData<TS>,
}

/// This implements the `new` method for `Samples<Finalized>`, returning `Samples<NonFinalized>`.
/// This seems odd, but it is convenient since `Finalized` is the default type.
impl Samples {
    #[must_use]
    pub fn new(sample_capacity: usize) -> Samples<NonFinalized> {
        Samples::<NonFinalized> {
            steps: Vec::with_capacity(sample_capacity),
            type_state: PhantomData::<NonFinalized>,
        }
    }
}

impl Samples<NonFinalized> {
    pub fn push_sample(&mut self, step: Step) {
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
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn step_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.steps.iter().map(Step::last_s)
    }

    #[must_use]
    pub fn at(&self, idx: usize) -> &Step {
        &self.steps[idx]
    }

    #[must_use]
    pub fn closest(&self, pos: &Position) -> Option<SampleIdxWithDistance> {
        self.steps
            .iter()
            .enumerate()
            .map(|(index, step)| SampleIdxWithDistance {
                distance: step.distance_to(pos),
                index,
            })
            .reduce(|a, b| if a.distance < b.distance { a } else { b })
    }
}

#[derive(Clone, Copy)]
pub struct SampleIdxWithDistance {
    pub distance: f32,
    pub index: usize,
}
