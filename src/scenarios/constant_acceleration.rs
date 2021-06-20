use super::{core::AccelerationField, import::Vec3};

pub struct ConstantAcceleration;

impl AccelerationField for ConstantAcceleration {
    fn value_at(&self, _pos: Vec3) -> Vec3 {
        Vec3::new(0., -1., 0.)
    }

    fn label(&self) -> String {
        "Constant Acceleration".to_string()
    }
}
