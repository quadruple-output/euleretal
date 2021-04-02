use std::{marker::PhantomData, ops::Index};

use crate::prelude::*;

pub type Point = Vec3;
pub type Acceleration = Vec3;
pub type Velocity = Vec3;

pub struct Samples<C: CalibrationPointConstraint> {
    steps: Vec<StepContext>,
    step_points: Vec<Point>,
    calibration_points_per_step: usize,
    calibration_points: Vec<CalibrationPoint>,
    point_dependencies: Vec<PointDependency>,
    calibration_point_constraint: PhantomData<C>,
}

pub trait CalibrationPointConstraint {}
pub enum WithoutCalibrationPoints {}
pub enum WithCalibrationPoints<const N: usize> {}
impl CalibrationPointConstraint for WithoutCalibrationPoints {}
impl<const N: usize> CalibrationPointConstraint for WithCalibrationPoints<N> {}

struct StepContext {
    time: R32,
    dt: R32,
    velocity: Velocity,
    acceleration: Acceleration,
}

#[derive(Clone)]
pub struct CalibrationPoint {
    pub position: Point,
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
        todo!()
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

impl<C: CalibrationPointConstraint> Samples<C> {
    pub fn step_points(&self) -> &Vec<Point> {
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

pub struct NewSample {
    pub time: R32,
    pub dt: R32,
    pub position: Point,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

pub struct NewSampleWithPoints<const N: usize> {
    pub time: R32,
    pub dt: R32,
    pub position: Point,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    /// Step Number
    pub n: usize,
    /// Time
    pub t: R32,
    /// delta t:
    pub dt: R32,
    /// Position
    pub s: Vec3,
    /// Velocity
    pub v: Vec3,
    /// Acceleration
    pub a: Vec3,
}

impl From<(usize, R32, R32, Vec3, Vec3, Vec3)> for Sample {
    fn from(tuple: (usize, R32, R32, Vec3, Vec3, Vec3)) -> Self {
        Self {
            n: tuple.0,
            t: tuple.1,
            dt: tuple.2,
            s: tuple.3,
            v: tuple.4,
            a: tuple.5,
        }
    }
}
