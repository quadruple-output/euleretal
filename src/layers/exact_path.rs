use crate::{Canvas, Scenario, StepSize, UiState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_exact_path.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
pub fn render_exact_path(
    ui_state: ResMut<UiState>,
    mut canvases: Query<&mut Canvas>, // always request canvases with 'mut'
    //integrations: Query<&Integration>,
    step_sizes: Query<&StepSize>,
    scenarios: Query<&Scenario>,
) {
    if let Some(min_dt) = step_sizes.iter().map(|step_size| step_size.dt.get()).min()
    // the desire to be able to use the previous call to min() was the trigger to use decorum::R32
    {
        for mut canvas in canvases.iter_mut() {
            let first_time = !canvas.has_trajectory();
            let scenario = canvas.get_scenario(&scenarios).unwrap();
            canvas.update_trajectory(&scenario, min_dt);
            if first_time {
                let todo = "autofocus should consider all samples, not just trajectory";
                canvas.auto_focus();
            }
            canvas.draw_trajectory(ui_state.strokes.trajectory);
        }
    }
}
