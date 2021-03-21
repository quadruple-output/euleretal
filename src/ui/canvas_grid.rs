use super::canvas_view;
use crate::prelude::*;
use bevy_ecs::World;
use egui::Ui;

pub fn show(ui: &mut Ui, world: &mut World) {
    let panel_size = ui.available_size_before_wrap_finite();
    let canvas_count = world.query_mut::<&mut canvas::comp::State>().count();
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    for mut canvas in world.query_mut::<&mut canvas::comp::State>() {
        canvas_view::show(ui, &mut *canvas, view_size);
    }
}
