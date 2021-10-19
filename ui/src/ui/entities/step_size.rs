use super::{core::Duration, misc::UserLabel, ui_import::egui};

pub struct StepSize {
    pub user_label: UserLabel,
    pub duration: Duration,
    pub color: egui::color::Hsva,
}

impl std::fmt::Display for StepSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.user_label.is_empty() {
            write!(f, "{}", self.duration)
        } else {
            write!(f, "{} \"{}\"", self.duration, self.user_label)
        }
    }
}
