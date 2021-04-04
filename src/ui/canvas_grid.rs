use super::canvas_view::{show_canvas, show_header_bar, CanvasOperation};
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
    let can_create = canvas_count < 4;
    let mut operation = CanvasOperation::Noop;

    for canvas_id in canvas_ids {
        let header_bar = show_header_bar(ui, canvas_id, world, can_close, can_create);
        if header_bar.inner != CanvasOperation::Noop {
            operation = header_bar.inner;
        }
        let inner_size = Vec2::new(view_size.x, view_size.y - header_bar.response.rect.height());
        show_canvas(ui, canvas_id, world, inner_size, control_state);
    }

    match operation {
        CanvasOperation::Create => {
            if let Some((any_scenario_id, _)) =
                world.query::<(bevy_ecs::Entity, &scenario::Kind)>().next()
            {
                let new_canvas_id = canvas::Bundle(
                    canvas::Kind,
                    canvas::State::new(),
                    scenario::Entity(any_scenario_id),
                )
                .spawn(world);
                if let Some((any_integrator_id, _)) = world
                    .query::<(bevy_ecs::Entity, &integrator::Kind)>()
                    .next()
                {
                    if let Some((any_step_size_id, _)) =
                        world.query::<(bevy_ecs::Entity, &step_size::Kind)>().next()
                    {
                        integration::Bundle(
                            integration::Kind,
                            integration::comp::State::new(integration::State::new()),
                            integrator::Entity(any_integrator_id),
                            step_size::Entity(any_step_size_id),
                            new_canvas_id,
                        )
                        .spawn(world);
                    }
                }
            }
        }
        CanvasOperation::Close { canvas_id } => {
            world.despawn(canvas_id).unwrap();
        }
        CanvasOperation::Noop => (),
    }
}
