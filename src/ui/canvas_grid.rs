use super::canvas_view::{show_canvas, show_header_bar, CanvasOperation};
use crate::prelude::*;
use bevy_ecs::{Entity, World};
use egui::Ui;

pub fn show(ui: &mut Ui, world: &mut World, control_state: &ControlState) {
    let panel_size = ui.available_size_before_wrap_finite();
    let canvas_count = world.query::<&canvas::Kind>().count();
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    let canvas_ids = world
        .query::<(Entity, &canvas::Kind)>()
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
        CanvasOperation::Create { source_canvas_id } => {
            if let Some((any_scenario_id, _)) = world.query::<(Entity, &scenario::Kind)>().next() {
                // new canvas:
                let new_canvas_id = canvas::Bundle(
                    canvas::Kind,
                    canvas::State::new(),
                    scenario::Entity(any_scenario_id),
                )
                .spawn(world);
                // copy canvas integrations:
                let source_canvas_integrations = world
                    .query::<(
                        &integration::Kind,
                        &integration::comp::IntegratorId,
                        &integration::comp::StepSizeId,
                        &integration::comp::CanvasId,
                    )>()
                    .filter(|(_, _, _, integration_canvas_id)| {
                        integration_canvas_id.0 == source_canvas_id
                    })
                    .map(|(_, integrator_id, step_size_id, _)| (*integrator_id, *step_size_id))
                    .collect::<Vec<_>>();
                for (integrator_id, step_size_id) in source_canvas_integrations {
                    integration::Bundle(
                        integration::Kind,
                        integration::comp::State::new(integration::State::new()),
                        integrator_id,
                        step_size_id,
                        new_canvas_id,
                    )
                    .spawn(world);
                }
            }
        }

        CanvasOperation::Close { canvas_id } => {
            let dependent_entities = world
                .query::<(Entity, &integration::comp::CanvasId)>()
                .filter(|(_, dependent_canvas_id)| dependent_canvas_id.0 == canvas_id)
                .map(|(entity, _)| entity)
                .collect::<Vec<_>>();
            for dependent_entity in dependent_entities {
                world.despawn(dependent_entity).unwrap();
            }
            world.despawn(canvas_id).unwrap();
        }
        CanvasOperation::Noop => (),
    }
}
