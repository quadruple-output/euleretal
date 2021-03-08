use super::canvas_view;
use crate::prelude::*;
use ::bevy::ecs::Query;
use ::egui::Ui;

pub fn show(ui: &mut Ui, canvases: &mut Query<&mut canvas::comp::State>) {
    let panel_size = ui.available_size_before_wrap_finite();
    let canvas_count = canvases.iter_mut().count();
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    for mut canvas in canvases.iter_mut() {
        canvas_view::show(ui, &mut *canvas, view_size);
    }
}
