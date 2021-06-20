use super::{misc, ui_import, ui_import::Ui};

mod colors;
mod number_precision;

pub fn show(ui: &mut Ui, settings: &mut misc::Settings) {
    ui.collapsing("Colors", |ui| {
        colors::show(ui, &mut settings.strokes);
    });
    ui.collapsing("Settings", |ui| {
        number_precision::show(ui, settings);
    });
}
