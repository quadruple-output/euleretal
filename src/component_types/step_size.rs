use crate::prelude::*;

// todo: move StepSize to ui::entities, and let the `Integration` only keep a reference to the Duration
pub struct StepSize {
    pub user_label: UserLabel,
    pub duration: Duration,
    pub color: egui::color::Hsva,
}

impl std::fmt::Display for StepSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.user_label.is_empty() {
            write!(f, "{}", self.duration.get())
        } else {
            write!(f, "{} \"{}\"", self.duration.get(), self.user_label)
        }
    }
}