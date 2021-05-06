use super::ControlState;
use crate::egui::{Slider, Ui};

pub fn show(ui: &mut Ui, state: &mut ControlState) {
    ui.horizontal(|ui| {
        ui.label("Display Decimals");
        ui.add(Slider::new(&mut state.format_precision, 0..=12));
        ui.label(format!("{}", state.format_precision));
    });
}
