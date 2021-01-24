#[derive(PartialEq)]
pub enum Scenario {
    LinearAccel,
    Rotation,
}

impl Default for Scenario {
    fn default() -> Self {
        Self::Rotation
    }
}
