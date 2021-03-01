use crate::prelude::*;
use ::egui::Slider;

pub struct Duration(pub ChangeTracker<R32>);

impl Duration {
    pub fn show_controls(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let mut duration_for_edit = self.0.get().into_inner();
            ui.add(
                Slider::f32(&mut duration_for_edit, 0.1..=50.)
                    .logarithmic(true)
                    .text("duration"),
            );
            self.0.set(duration_for_edit.into());
        });
    }
}
