use super::super::{Canvas, ControlState};
use crate::prelude::*;

pub fn render(state: &ControlState, canvas: &Obj<Canvas>, painter: &egui::Painter) {
    let min_dt = canvas
        .borrow()
        .integrations()
        .map(|integration| integration.borrow().step_size.borrow().duration.get())
        .min() // this crate depends on decorum::R32 just to be able to use this min() function
        .unwrap_or_else(|| 0.1.into());

    let scenario_obj = Rc::clone(canvas.borrow().scenario());
    let scenario = scenario_obj.borrow();
    let first_time = !canvas.borrow().has_trajectory();
    canvas.borrow_mut().update_trajectory(min_dt);
    canvas.borrow().integrations().for_each(|integration| {
        let mut integration = integration.borrow_mut();
        if first_time {
            integration.reset();
        }
        integration.update(
            // todo: replace 4 params by a single one
            &*scenario.acceleration,
            &scenario.start_position,
            &scenario.start_velocity,
            &scenario.duration,
        );
    });
    if first_time {
        let bbox = canvas.borrow().bbox(); // need this extra assignment to drop the borrowed canvas
        if let Some(mut bbox) = bbox {
            canvas
                .borrow()
                .integrations()
                .for_each(|integration| integration.borrow().stretch_bbox(&mut bbox));
            canvas.borrow_mut().set_visible_bbox(&bbox);
        }
    }

    canvas
        .borrow()
        .draw_trajectory(state.strokes.trajectory, painter);
    canvas.borrow().integrations().for_each(|integration| {
        integration.borrow().draw_on(
            &canvas.borrow(),
            Color32::from(integration.borrow().step_size.borrow().color),
            integration.borrow().integrator_conf.borrow().stroke,
            painter,
        );
    });
}
