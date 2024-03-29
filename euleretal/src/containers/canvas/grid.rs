use super::{
    entities::Canvas,
    ui_import::{Ui, Vec2},
    view::{self, CanvasOperation},
    World,
};

pub fn show(ui: &mut Ui, world: &mut World) {
    let panel_size = ui.available_size_before_wrap();
    let canvas_count = world.canvases().count();
    #[allow(clippy::cast_precision_loss)]
    let view_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
    let can_close = canvas_count > 1;
    let can_create = canvas_count < 4;
    let mut operation = CanvasOperation::Noop;

    world.canvases().for_each(|canvas| {
        let header_bar = view::show_header_bar(ui, canvas, world, can_close, can_create);
        if let CanvasOperation::Noop = header_bar.inner {
        } else {
            operation = header_bar.inner;
        }
        let inner_size = Vec2::new(view_size.x, view_size.y - header_bar.response.rect.height());
        view::show_canvas(ui, canvas, inner_size, world);
    });

    match operation {
        CanvasOperation::Create { source_canvas } => {
            let mut new_canvas;
            {
                let source_canvas = source_canvas.borrow();
                new_canvas = Canvas::new(source_canvas.scenario_idx());
                // copy canvas integrations:
                source_canvas.integrations().for_each(|integration| {
                    let integration = integration.borrow();
                    new_canvas.add_integration(integration.clone());
                });
            }
            world.add_canvas(new_canvas);
        }

        CanvasOperation::Close { canvas } => {
            world.remove_canvas(canvas);
        }
        CanvasOperation::Noop => (),
    }
}
