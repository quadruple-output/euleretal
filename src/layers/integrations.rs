use crate::{Canvas, ConfiguredIntegrator, Integration, Scenario, StepSize, UiState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_integrations.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
pub fn render_integrations(
    ui_state: ResMut<UiState>,
    mut canvases: Query<(Entity, &mut Canvas)>, // always request canvases with 'mut'
    mut integrations: Query<&mut Integration>,
    integrators: Query<&ConfiguredIntegrator>,
    step_sizes: Query<&StepSize>,
    scenarios: Query<&Scenario>,
) {
    for (canvas_id, mut canvas) in canvases.iter_mut() {
        let scenario = canvas.get_scenario(&scenarios).unwrap();
        // calculate minimum of all step_sizes for this canvas:
        if let Some(min_dt) = integrations
            .iter_mut()
            .filter(|integration| integration.get_canvas_id() == canvas_id)
            .map(|mut integration| {
                let integrator = integration.get_integrator(&integrators).unwrap();
                let step_size = integration.get_step_size(&step_sizes).unwrap();
                integration.update(&scenario, &integrator, &step_size);
                integration.draw_on(&mut canvas, step_size.color.into(), integrator.stroke);
                step_size
            })
            .map(|step_size| step_size.dt.get())
            .min()
        // (this crate depends on decorum::R32 just to be able to use this min() function)
        {
            let first_time = !canvas.has_trajectory();
            canvas.update_trajectory(&scenario, min_dt);
            if first_time {
                let todo = "autofocus should consider all samples, not just trajectory";
                canvas.auto_focus();
            }
            canvas.draw_trajectory(ui_state.strokes.trajectory);
        } else {
            warn!("no integration for canvas_id {:?}", canvas_id);
        }
    }
}
