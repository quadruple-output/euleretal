pub struct StepSize {
    pub label: String,
    pub dt: f32,
}

impl StepSize {
    pub fn new(label: &str, dt: f32) -> Self {
        Self {
            label: label.to_string(),
            dt,
        }
    }
}
