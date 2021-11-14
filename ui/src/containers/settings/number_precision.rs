use super::{
    misc::Settings,
    ui_import::{egui::Slider, Ui},
};

pub fn show(ui: &mut Ui, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.label("Display Decimals");
        ui.add(Slider::new(&mut settings.format_precision, 0..=12));
        ui.label(format!("{}", settings.format_precision));
    });
}
