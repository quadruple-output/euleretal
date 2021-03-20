use crate::app::prelude::*;

#[derive(Debug, Default)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn expand_to(&mut self, s: Vec3) {
        self.min.x = self.min.x.min(s.x);
        self.min.y = self.min.y.min(s.y);
        self.min.z = self.min.z.min(s.z);
        self.max.x = self.max.x.max(s.x);
        self.max.y = self.max.y.max(s.y);
        self.max.z = self.max.z.max(s.z);
    }

    pub fn center(&self) -> Vec3 {
        0.5 * (self.max + self.min)
    }

    pub fn diameter(&self) -> f32 {
        (self.max.x - self.min.x)
            .max(self.max.y - self.min.y)
            .max(self.max.z - self.min.z)
    }
}
