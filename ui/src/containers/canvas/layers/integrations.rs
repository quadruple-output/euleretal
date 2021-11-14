use super::{entities::CanvasPainter, World};

pub fn render(canvas: &mut CanvasPainter, world: &World) {
    let mut start = ::std::time::Instant::now();
    let mut updated = false;

    let min_dt = canvas
        .map_integrations(|integration| integration.step_size.borrow().duration)
        .min() // this crate depends on R32 just to be able to use this min() function
        .unwrap_or_else(|| 0.1.into());

    let first_time = !canvas.has_trajectory();
    let scenario = world.scenarios()[canvas.scenario_idx()].borrow();
    canvas.update_trajectory(&scenario, min_dt);
    {
        canvas.for_each_integration_mut(|mut integration| {
            if first_time {
                integration.reset();
            }
            updated |= integration.update(&scenario);
        });
    }
    if updated {
        log::debug!(
            "Render Canvas: integrate: {}µs",
            start.elapsed().as_micros()
        );
        start = ::std::time::Instant::now();
    }
    if first_time {
        canvas.update_bounding_box();
    }

    canvas.draw_trajectory(world.settings.strokes.trajectory);
    canvas.for_each_integration(|integration| {
        integration.draw_on(canvas, &world.settings);
    });
    if updated {
        log::debug!("Render Canvas: draw: {}µs", start.elapsed().as_micros());
    }
}
