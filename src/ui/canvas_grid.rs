use super::canvas_view::{show_canvas, show_header_bar, CanvasOperation};
use crate::{entities::integration::Integration, prelude::*};
use egui::Ui;
use std::rc::Rc;

pub fn show(ui: &mut Ui, world: &mut World, control_state: &ControlState) {
    let panel_size = ui.available_size_before_wrap_finite();
    let canvas_count = world.canvases().count();
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    let can_close = canvas_count > 1;
    let can_create = canvas_count < 4;
    let mut operation = CanvasOperation::Noop;

    world.canvases().for_each(|canvas| {
        let header_bar = show_header_bar(ui, canvas, world, can_close, can_create);
        if let CanvasOperation::Noop = header_bar.inner {
        } else {
            operation = header_bar.inner;
        }
        let inner_size = Vec2::new(view_size.x, view_size.y - header_bar.response.rect.height());
        show_canvas(ui, canvas, inner_size, control_state);
    });

    match operation {
        CanvasOperation::Create { source_canvas } => {
            let first_scenario = world.scenarios().next().map(|scenario| Rc::clone(scenario));
            if let Some(any_scenario) = first_scenario {
                // new canvas:
                let mut new_canvas = world
                    .add_canvas(Canvas::new(Rc::clone(&any_scenario)))
                    .borrow_mut();
                // copy canvas integrations:
                source_canvas
                    .borrow()
                    .integrations()
                    .for_each(|integration| {
                        let integration = integration.borrow();
                        new_canvas.add_integration(Integration::new(
                            Rc::clone(&integration.integrator_conf),
                            Rc::clone(&integration.step_size),
                        ));
                    });
            }
        }

        CanvasOperation::Close { canvas } => {
            world.remove_canvas(canvas);
        }
        CanvasOperation::Noop => (),
    }
}
