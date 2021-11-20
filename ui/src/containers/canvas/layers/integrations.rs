use super::{entities::CanvasPainter, World};

pub fn render(canvas: &mut CanvasPainter, world: &World) {
    canvas.draw_trajectory(world.settings.strokes.trajectory);
    canvas.for_each_integration(|integration| {
        integration.draw_on(canvas, world);
    });
}
