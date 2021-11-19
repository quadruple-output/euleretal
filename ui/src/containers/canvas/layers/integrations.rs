use super::{entities::CanvasPainter, World};

pub fn render(canvas: &mut CanvasPainter, world: &World) {
    #[cfg(not(target_arch = "wasm32"))]
    let mut start = ::std::time::Instant::now();

    let mut updated = false;

    {
        /* TODO: this block goes into canvas.update_model();
                 and shall be implemented by `Canvas` (not Painter!)
                 and shall be called in crate:containers::canvas::view::show_canvas()
        */
        let min_dt = canvas
            .map_integrations(|integration| integration.fetch_step_duration(world))
            .min() // this crate depends on R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let scenario_is_new = canvas.scenario_is_new_once();
        let scenario = world.scenarios()[canvas.scenario_idx()].borrow();
        canvas.update_trajectory(&scenario, min_dt);
        canvas.for_each_integration_mut(|mut integration| {
            if scenario_is_new {
                integration.reset();
            }
            let integrator = &*world[integration.integrator_idx()].borrow().core;
            let step_duration = world[integration.step_size_idx()].borrow().duration;
            updated |= integration.update(&scenario, integrator, step_duration);
        });

        #[cfg(not(target_arch = "wasm32"))]
        if updated {
            log::debug!(
                "Render Canvas: integrate: {}µs",
                start.elapsed().as_micros()
            );
            start = ::std::time::Instant::now();
        }

        if scenario_is_new {
            log::debug!("updating bounding box");
            canvas.update_bounding_box();
        }
    }

    canvas.draw_trajectory(world.settings.strokes.trajectory);
    canvas.for_each_integration(|integration| {
        let integrator_stroke = world[integration.integrator_idx()].borrow().stroke;
        integration.draw_on(canvas, integrator_stroke, world);
    });

    #[cfg(not(target_arch = "wasm32"))]
    if updated {
        log::debug!("Render Canvas: draw: {}µs", start.elapsed().as_micros());
    }
}
