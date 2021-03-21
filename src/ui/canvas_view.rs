use crate::prelude::*;
use egui::Ui;

pub fn show(ui: &mut Ui, canvas: &mut canvas::comp::State, size: Vec2) {
    ui.vertical(|ui| {
        let response = ui.horizontal(|ui| {
            let todo = "add canvas view header information";
            ui.label("<put header info here>");
        });

        let inner_size = Vec2::new(size.x, size.y - response.response.rect.height());
        canvas.allocate_painter(ui, inner_size);
    });
}
