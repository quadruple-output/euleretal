#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::multiple_crate_versions)]
#![allow(incomplete_features)] // for the two `const_*` features below
#![feature(const_evaluatable_checked)] // used by DtFraction
#![feature(const_generics)] // used by DtFraction

mod import {
    pub use ::parry3d::{query::PointQuery, shape};
    pub type OrderedF32 = ::ordered_float::OrderedFloat<f32>;
    pub type Point3 = ::parry3d::math::Point<f32>;
    pub type Vec3 = ::parry3d::math::Vector<f32>;
}

mod acceleration;
mod acceleration_field;
mod duration;
mod fraction;
mod integration;
pub mod integration_step;
mod integrator;
mod r#move;
mod obj;
mod position;
pub mod samples;
mod scenario;
mod vector_quantity;
mod velocity;

pub use acceleration::Acceleration;
pub use acceleration_field::AccelerationField;
pub use duration::Duration;
pub use fraction::Fraction;
pub use import::{Point3, Vec3};
pub use integration::Integration;
pub use integration_step::{StartCondition, Step};
pub use integrator::Integrator;
pub use obj::Obj;
pub use position::Position;
pub use r#move::Move;
pub use samples::Samples;
pub use scenario::Scenario;
use vector_quantity::VectorQuantity;
pub use velocity::Velocity;
pub enum PhysicalQuantityKind {
    Position,
    Velocity,
    Acceleration,
}
