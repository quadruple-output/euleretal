use super::core::Position;

#[derive(Debug)]
pub struct BoundingBox {
    min: Position,
    max: Position,
}

impl BoundingBox {
    pub fn new_at(center: &Position) -> Self {
        Self {
            min: *center,
            max: *center,
        }
    }

    pub fn expand_to(&mut self, s: &Position) {
        self.min = self.min.inf(s);
        self.max = self.max.sup(s);
    }

    #[must_use]
    pub fn center(&self) -> Position {
        Position::from(0.5 * (self.max.coords + self.min.coords))
    }

    #[must_use]
    pub fn diameter(&self) -> f32 {
        let min2max = self.max - self.min;
        min2max.x.max(min2max.y).max(min2max.z)
    }
}
