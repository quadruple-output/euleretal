use super::{core::Duration, ui_import::egui};

// todo: move StepSize to ui::entities, and let the `Integration` only keep a reference to the Duration
pub struct StepSize {
    pub user_label: crate::ui::UserLabel,
    pub duration: Duration,
    pub color: egui::color::Hsva,
}

impl std::fmt::Display for StepSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.user_label.is_empty() {
            write!(f, "{}", self.duration.0)
        } else {
            write!(f, "{} \"{}\"", self.duration.0, self.user_label)
        }
    }
}
