use egui::{
    color::Hsva,
    color_picker::{color_edit_button_hsva, Alpha},
    Slider, Ui,
};

pub struct StepSize {
    pub label: String,
    pub dt: f32,
    pub color: Hsva,
}

impl StepSize {
    pub fn new(label: &str, dt: f32, color: Hsva) -> Self {
        Self {
            label: label.to_string(),
            dt,
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
            ui.add(
                Slider::f32(&mut self.dt, 0.01..=2.)
                    .text("dt")
                    .logarithmic(true),
            );
            if self.dt < 0.01 {
                self.dt = 0.01;
            }
        });
    }
}
