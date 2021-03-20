use crate::prelude::*;

pub trait Acceleration: Send + Sync {
    fn value_at(&self, pos: Vec3) -> Vec3;

    fn label(&self) -> String;
}
