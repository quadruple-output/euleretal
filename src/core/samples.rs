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
    steps: Vec<StepContext>,
    step_points: Vec<Position>,
    calibration_points_per_step: usize,
    calibration_points: Vec<CalibrationPoint>,
    calibration_point_constraint: PhantomData<TS>,
}

struct StepContext {
    time: R32,
    dt: R32,
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
            step_points: Vec::with_capacity(sample_capacity + 1),
            calibration_points_per_step,
            calibration_points: Vec::with_capacity(calibration_points_per_step * sample_capacity),
            calibration_point_constraint: PhantomData::<NonFinalized>,
        };
        instance.steps.push(StepContext {
            time: 0.0.into(), // start time is always zero
            dt: 0.0.into(),   // initial sample has no delta
            velocity: start_condition.velocity,
            acceleration: start_condition.acceleration,
        });
        instance.step_points.push(start_condition.position);
        instance
    }
}

impl Samples<NonFinalized> {
    pub fn push_sample(&mut self, sample: &NewSampleWithPoints) {
        self.step_points.push(sample.position);
        self.steps.push(StepContext {
            time: self.steps.last().unwrap().time + sample.dt,
            dt: sample.dt,
            velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
        for calibration_point in &sample.calibration_points {
            self.calibration_points.push(calibration_point.clone());
        }
    }

    #[must_use]
    pub fn current(&self) -> Option<StartCondition> {
        if let (Some(current_step), Some(current_point)) =
            (self.steps.last(), self.step_points.last())
        {
            Some(StartCondition {
                position: *current_point,
                velocity: current_step.velocity,
                acceleration: current_step.acceleration,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn finalized(self) -> Samples<Finalized> {
        Samples {
            steps: self.steps,
            step_points: self.step_points,
            calibration_points_per_step: self.calibration_points_per_step,
            calibration_points: self.calibration_points,
            calibration_point_constraint: PhantomData::<Finalized>,
        }
    }
}

impl Samples<Finalized> {
    #[must_use]
    pub fn step_points(&self) -> &Vec<Position> {
        &self.step_points
    }

    #[must_use]
    pub fn at(&self, idx: usize) -> CompleteSample {
        let step = &self.steps[idx];
        CompleteSample {
            n: idx,
            t: step.time,
            dt: step.dt,
            v: step.velocity,
            a: step.acceleration,
            s: self.step_points[idx],
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
