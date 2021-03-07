use super::State;
use ::egui::{Slider, Ui};

pub fn show(ui: &mut Ui, state: &mut State) {
    ui.horizontal(|ui| {
        ui.label("Display Decimals");
        ui.add(Slider::usize(&mut state.format_precision, 0..=12));
        ui.label(format!("{}", state.format_precision));
    });
}
