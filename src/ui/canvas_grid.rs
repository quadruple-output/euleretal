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
        .collect::<Vec<_>>(); // have to `collect()` because `mut world` is used in loop below
    let can_close = canvas_count > 1;
    let mut canvas_to_close = None;
    for canvas_id in canvas_ids {
        let header_bar = canvas_view::show_header_bar(ui, canvas_id, world, can_close);
        header_bar.inner.map(|close_button| {
            close_button
                .clicked()
                .then(|| canvas_to_close = Some(canvas_id))
        });
        let inner_size = Vec2::new(view_size.x, view_size.y - header_bar.response.rect.height());
        canvas_view::show_canvas(ui, canvas_id, world, inner_size, control_state);
    }
    if let Some(canvas_to_close) = canvas_to_close {
        world.despawn(canvas_to_close).unwrap();
    }
}
