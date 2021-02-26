use crate::{ChangeTracker, TrackedChange};
use decorum::R32;
use egui::{
    color::Hsva,
    color_picker::{color_edit_button_hsva, Alpha},
    Slider, Ui,
};

pub struct StepSize {
    pub label: String,
    pub dt: ChangeTracker<R32>,
    pub color: Hsva,
}

impl TrackedChange for StepSize {
    fn change_count(&self) -> crate::change_tracker::ChangeCount {
        self.dt.change_count()
    }
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::prelude::Entity);

impl StepSize {
    pub fn new(label: &str, dt: R32, color: Hsva) -> Self {
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
            let mut dt = self.dt.get().into_inner();
            ui.add(Slider::f32(&mut dt, 0.01..=2.).text("dt").logarithmic(true));
            self.dt.set(R32::from(dt).max(R32::from(0.01)));
        });
    }
}
