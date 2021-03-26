use crate::prelude::*;
use egui::Ui;

use super::layers;

pub fn show(
    ui: &mut Ui,
    canvas_id: bevy_ecs::Entity,
    world: &mut World,
    size: Vec2,
    control_state: &ControlState,
) {
    ui.vertical(|ui| {
        let response = ui.horizontal(|ui| {
            let todo = "add canvas view header information";
            ui.label("<put header info here>");
        });

        let inner_size = Vec2::new(size.x, size.y - response.response.rect.height());
        let mut canvas = world.get_mut::<canvas::comp::State>(canvas_id).unwrap();
        let (response, painter) = canvas.allocate_painter(ui, inner_size);

        layers::acceleration_field::render(world, control_state, canvas_id, &response, &painter);
        layers::coordinates::render(world, &control_state, canvas_id, &response.rect, &painter);
        layers::integrations::render(world, &control_state, canvas_id, &painter);
        layers::inspector::render(world, &control_state, &response, &painter);
    });
}
