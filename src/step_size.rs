use crate::{ChangeTracker, TrackedChange};
use egui::{
    color::Hsva,
    color_picker::{color_edit_button_hsva, Alpha},
    Slider, Ui,
};

pub struct StepSize {
    pub label: String,
    pub dt: ChangeTracker<f32>,
    pub color: Hsva,
}

impl TrackedChange for StepSize {
    fn change_count(&self) -> crate::change_tracker::ChangeCount {
        self.dt.change_count()
    }
}

impl StepSize {
    pub fn new(label: &str, dt: f32, color: Hsva) -> Self {
        Self {
            label: label.to_string(),
            dt: ChangeTracker::with(dt),
            color,
        }
    }

    pub fn show_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|mut ui| {
            // edit color:
            color_edit_button_hsva(&mut ui, &mut self.color, Alpha::BlendOrAdditive);
            // edit label:
            ui.add(egui::TextEdit::singleline(&mut self.label).desired_width(0.));
            if self.label.is_empty() {
                self.label = "<unnamed>".to_string();
            }
            // edit dt:
            let mut dt = self.dt.get();
            ui.add(Slider::f32(&mut dt, 0.01..=2.).text("dt").logarithmic(true));
            self.dt.set(f32::max(dt, 0.01));
        });
    }
}
