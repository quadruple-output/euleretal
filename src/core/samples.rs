use crate::prelude::*;
use std::marker::PhantomData;

pub struct Samples<C: CalibrationPointConstraint = FinalizedCalibrationPoints> {
    steps: Vec<StepContext>,
    step_points: Vec<Position>,
    calibration_points_per_step: usize,
    calibration_points: Vec<CalibrationPoint>,
    point_dependencies: Vec<PointDependency>,
    calibration_point_constraint: PhantomData<C>,
}

pub trait CalibrationPointConstraint {}
pub trait ConcreteCalibrationPointConstraint: CalibrationPointConstraint {}

pub enum FinalizedCalibrationPoints {}
pub enum WithoutCalibrationPoints {}
pub enum WithCalibrationPoints<const N: usize> {}

impl CalibrationPointConstraint for FinalizedCalibrationPoints {}

impl ConcreteCalibrationPointConstraint for WithoutCalibrationPoints {}
impl CalibrationPointConstraint for WithoutCalibrationPoints {}

impl<const N: usize> ConcreteCalibrationPointConstraint for WithCalibrationPoints<N> {}
impl<const N: usize> CalibrationPointConstraint for WithCalibrationPoints<N> {}

struct StepContext {
    time: R32,
    dt: R32,
    velocity: Velocity,
    acceleration: Acceleration,
}

#[derive(Clone)]
pub struct CalibrationPoint {
    pub position: Position,
    pub dt: Fraction,
    pub acceleration: Acceleration,
    // todo: do we need a Velocity here?
}

struct PointDependency {
    predecessor: Predecessor,
    successor: Successor,
    weight: usize,
}

enum Predecessor {
    StartingPoint,
    CalibrationPoint(usize),
}

enum Successor {
    EndPoint,
    CalibrationPoint(usize),
}

#[derive(Clone)]
pub struct Fraction {
    pub numerator: usize,
    pub denominator: usize,
}

impl Samples<WithoutCalibrationPoints> {
    pub fn with_capacity(capa_samples: usize) -> Self {
        Self {
            steps: Vec::with_capacity(capa_samples + 1), // +1 for the Endpoint
            step_points: Vec::with_capacity(capa_samples),
            calibration_points_per_step: 0, // set by set_point_dependencies()
            calibration_points: Vec::with_capacity(0),
            point_dependencies: Vec::with_capacity(0),
            calibration_point_constraint: PhantomData::<WithoutCalibrationPoints>,
        }
    }

    pub fn push_sample(&mut self, sample: &NewSample) {
        self.step_points.push(sample.position);
        self.steps.push(StepContext {
            time: sample.time,
            dt: sample.dt,
            velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
    }
}

impl<const N: usize> Samples<WithCalibrationPoints<N>> {
    pub fn with_capacity(capa_samples: usize) -> Self {
        Self {
            steps: Vec::with_capacity(capa_samples + 1), // +1 for the Endpoint
            step_points: Vec::with_capacity(capa_samples),
            calibration_points_per_step: N,
            calibration_points: Vec::with_capacity(N * capa_samples),
            // as a HEURISTIC, we assume that each point is calculated from two predecessors:
            point_dependencies: Vec::with_capacity(N * 2),
            calibration_point_constraint: PhantomData::<WithCalibrationPoints<N>>,
        }
    }

    pub fn add_dependency(&mut self, dependency: PointDependency) {
        self.point_dependencies.push(dependency);
    }

    pub fn push_sample(&mut self, sample: &NewSampleWithPoints<N>) {
        self.step_points.push(sample.position);
        self.steps.push(StepContext {
            time: sample.time,
            dt: sample.dt,
            velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
        for calibration_point in &sample.calibration_points {
            self.calibration_points.push(calibration_point.clone());
        }
    }
}

impl Samples<FinalizedCalibrationPoints> {
    pub fn step_points(&self) -> &Vec<Position> {
        &self.step_points
    }

    pub fn at(&self, idx: usize) -> CompleteSample {
        let step = &self.steps[idx];
        let result = CompleteSample {
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
        };
        result
    }
}

impl<C: ConcreteCalibrationPointConstraint> Samples<C> {
    pub fn finalize(self, sample: NewSample) -> Samples<FinalizedCalibrationPoints> {
        self.step_points.push(sample.position);
        self.steps.push(StepContext {
            time: sample.time,
            dt: sample.dt,
            velocity: sample.velocity,
            acceleration: sample.acceleration,
        });
        Samples {
            steps: self.steps,
            step_points: self.step_points,
            calibration_points_per_step: self.calibration_points_per_step,
            calibration_points: self.calibration_points,
            point_dependencies: self.point_dependencies,
            calibration_point_constraint: PhantomData::<FinalizedCalibrationPoints>,
        }
    }
}

pub struct NewSample {
    pub time: R32,
    pub dt: R32,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

pub struct NewSampleWithPoints<const N: usize> {
    pub time: R32,
    pub dt: R32,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub calibration_points: [CalibrationPoint; N],
}

pub struct CompleteSample<'a> {
    /// Step Number
    pub n: usize,
    /// Time
    pub t: R32,
    /// delta t:
    pub dt: R32,
    /// Velocity
    pub v: Vec3,
    /// Acceleration
    pub a: Vec3,
    /// Position
    pub s: Vec3,
    pub calibration_points: Vec<&'a CalibrationPoint>,
}
