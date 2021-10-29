use crate::{PhysicalQuantityKind, Position, Vec3};

pub trait Contribution {
    fn sampling_position(&self) -> Position;

    fn kind(&self) -> PhysicalQuantityKind;

    fn vector(&self) -> Option<Vec3>;

    fn contributions_factor(&self) -> f32;

    fn contributions_iter(&self) -> Box<dyn Iterator<Item = Box<dyn Contribution + '_>> + '_>;
}
