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
            color_edit_button_hsva(&mut ui, &mut self.color, Alpha::BlendOrAdditive);
            ui.text_edit_singleline(&mut self.label);
            // ui.add(egui::DragValue::f32(&mut self.dt).fixed_decimals(2));
            // if self.dt < 0.001 {
            //     self.dt = 0.01;
            // }
            let mut ln_dt = self.dt.ln();
            ui.add(Slider::f32(&mut ln_dt, 0.01f32.ln()..=2.0f32.ln()));
            self.dt = ln_dt.exp();
            ui.label(format!("{:.4}", self.dt));
            //
            //ui.add(Slider::f32(&mut self.dt, 0.01..=2.));
            //
            // let mut dt: String = format!("{}", self.dt);
            // ui.text_edit_singleline(&mut dt);
            // if let Ok(dt) = dt.parse::<f32>() {
            //     self.dt = dt;
            // } else if let Ok(dt) = dt.parse::<usize>() {
            //     self.dt = dt as f32
            // };
        });
    }
}
