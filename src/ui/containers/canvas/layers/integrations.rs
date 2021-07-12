use super::{entities::CanvasPainter, misc::settings};

pub fn render(strokes: &settings::Strokes, canvas: &mut CanvasPainter) {
    let min_dt = canvas
        .map_integrations(|integration| integration.step_size.borrow().duration.0)
        .min() // this crate depends on decorum::R32 just to be able to use this min() function
        .unwrap_or_else(|| 0.1.into());

    let first_time = !canvas.has_trajectory();
    canvas.update_trajectory(min_dt);
    let scenario_obj = canvas.scenario(); // need explicit `let` to extend lifetime of the owned value
    let scenario = scenario_obj.borrow();
    canvas.for_each_integration_mut(|mut integration| {
        if first_time {
            integration.reset();
        }
        integration.update(&*scenario);
    });
    if first_time {
        canvas.update_bounding_box();
    }

    canvas.draw_trajectory(strokes.trajectory);
    canvas.for_each_integration(|integration| {
        integration.draw_on(canvas);
    });
}
