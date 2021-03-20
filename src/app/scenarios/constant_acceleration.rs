use crate::prelude::*;

pub struct ConstantAcceleration;

impl Acceleration for ConstantAcceleration {
    fn value_at(&self, _pos: Vec3) -> Vec3 {
        Vec3::new(0., -1., 0.)
    }

    fn label(&self) -> String {
        "Constant Acceleration".to_string()
    }
}
