use crate::prelude::*;

pub type Point = Vec3;
pub type Acceleration = Vec3;
pub type Velocity = Vec3;

pub struct Samples {
    steps: Vec<Step>,
    step_points: Vec<Point>,
    aux_points: Vec<Point>,
    aux_points_per_step: usize,
    aux_point_dependencies: Vec<PointDependency>,
}

struct Step {
    time: R32,
    duration: R32,
    velocity: Velocity,
    acceleration: Acceleration,
}

struct PointDependency {
    predecessor_idx: usize,
    successor_idx: usize,
    weight: Fraction,
}

struct Fraction {
    numerator: usize,
    denominator: usize,
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
