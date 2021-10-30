use super::import::Point3;

#[derive(Debug)]
pub struct BoundingBox {
    min: Point3,
    max: Point3,
}

impl BoundingBox {
    pub fn new_at(center: impl Into<Point3>) -> Self {
        let center = center.into();
        Self {
            min: center,
            max: center,
        }
    }

    pub fn expand_to(&mut self, p: impl Into<Point3>) {
        let p = p.into();
        self.min = self.min.inf(&p);
        self.max = self.max.sup(&p);
    }

    #[must_use]
    pub fn center(&self) -> Point3 {
        Point3::from(0.5 * (self.max.coords + self.min.coords))
    }

    #[must_use]
    pub fn diameter(&self) -> f32 {
        let min2max = self.max - self.min;
        min2max.x.max(min2max.y).max(min2max.z)
    }
}
