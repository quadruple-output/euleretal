use super::canvas_view;
use crate::prelude::*;
use bevy_ecs::World;
use egui::Ui;

pub fn show(ui: &mut Ui, world: &mut World, control_state: &ControlState) {
    let panel_size = ui.available_size_before_wrap_finite();
    let canvas_count = world.query::<&canvas::Kind>().count();
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    let canvas_ids = world
        .query::<(bevy_ecs::Entity, &canvas::Kind)>()
        .map(|(canvas_id, _)| canvas_id)
        .collect::<Vec<_>>();
    for canvas_id in canvas_ids {
        canvas_view::show(ui, canvas_id, world, view_size, control_state);
    }
}
