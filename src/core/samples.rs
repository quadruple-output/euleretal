use super::{import::R32, Acceleration, Fraction, Position, Velocity};
use ::std::marker::PhantomData;
use type_state::{Finalized, NonFinalized, TypeState};

mod type_state {
    pub trait TypeState {}

    pub struct Finalized {}
    pub struct NonFinalized {}

    impl TypeState for Finalized {}
    impl TypeState for NonFinalized {}
}

pub struct Samples<TS: TypeState = Finalized> {
    steps: Vec<Step>,
    calibration_points_per_step: usize,
    calibration_points: Vec<CalibrationPoint>,
    type_state: PhantomData<TS>,
}

#[doc(hidden)]
pub struct Step {
    time: R32,
    dt: R32,
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

#[derive(Clone, Default)]
pub struct CalibrationPoint {
    pub position: Position,
    /// fraction of dt for this point (starting point is `0/n`, end point is `n/n`)
    pub dt_fraction: Fraction,
    /// absolute acceleration
    pub acceleration: Option<Acceleration>,
    /// effectively used proportion of acceleration
    pub eff_acceleration: Option<Acceleration>,
    /// absolute velocity
    pub velocity: Option<Velocity>,
    /// effectively used proportion of velocity
    pub eff_velocity: Option<Velocity>,
}

/// This implements the `new` method for `Samples<Finalized>`, returning `Samples<NonFinalized>`.
/// This seems odd, but it is convenient since `Finalized` is the default type.
impl Samples {
    pub fn new(
        start_condition: &StartCondition,
        calibration_points_per_step: usize,
        sample_capacity: usize,
    ) -> Samples<NonFinalized> {
        let mut instance = Samples::<NonFinalized> {
            steps: Vec::with_capacity(sample_capacity + 1), // +1 for the Endpoint
            calibration_points_per_step,
            calibration_points: Vec::with_capacity(calibration_points_per_step * sample_capacity),
            type_state: PhantomData::<NonFinalized>,
        };
        instance.steps.push(Step {
            time: 0.0.into(), // start time is always zero
            dt: 0.0.into(),   // initial sample has no delta
            position: start_condition.position,
            velocity: start_condition.velocity,
            acceleration: start_condition.acceleration,
        });
        instance
    }
}

impl Samples<NonFinalized> {
    pub fn push_sample(&mut self, sample: &NewSampleWithPoints) {
        self.steps.push(Step {
            time: self.steps.last().unwrap().time + sample.dt,
            dt: sample.dt,
            position: sample.position,
            velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
        for calibration_point in &sample.calibration_points {
            self.calibration_points.push(calibration_point.clone());
        }
    }

    #[must_use]
    pub fn current(&self) -> Option<StartCondition> {
        self.steps.last().map(|current_step| StartCondition {
            position: current_step.position,
            velocity: current_step.velocity,
            acceleration: current_step.acceleration,
        })
    }

    #[must_use]
    pub fn finalized(self) -> Samples<Finalized> {
        Samples {
            steps: self.steps,
            calibration_points_per_step: self.calibration_points_per_step,
            calibration_points: self.calibration_points,
            type_state: PhantomData::<Finalized>,
        }
    }
}

impl Samples<Finalized> {
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn step_positions<'a>(
        &'a self,
    ) -> std::iter::Map<std::slice::Iter<'a, Step>, fn(&'a Step) -> Position> {
        let r = self
            .steps
            .iter()
            .map(Self::map_step_to_position as fn(_) -> _);
        r
    }

    fn map_step_to_position(step: &Step) -> Position {
        step.position
    }

    pub fn at(&self, idx: usize) -> CompleteSample {
        let step = &self.steps[idx];
        CompleteSample {
            n: idx,
            t: step.time,
            dt: step.dt,
            v: step.velocity,
            a: step.acceleration,
            s: step.position,
            calibration_points: self
                .calibration_points
                .iter()
                .skip(self.calibration_points_per_step * idx)
                .take(self.calibration_points_per_step)
                .collect(),
        }
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
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub calibration_points: Vec<CalibrationPoint>,
}

pub struct CompleteSample<'a> {
    /// Step Number
    pub n: usize,
    /// Time
    pub t: R32,
    /// delta t:
    pub dt: R32,
    /// Position
    pub s: Position,
    /// Velocity
    pub v: Velocity,
    /// Acceleration
    pub a: Acceleration,
    pub calibration_points: Vec<&'a CalibrationPoint>,
}
