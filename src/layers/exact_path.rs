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
    let mut min_dt = f32::MAX;
    for step_size in step_sizes.iter() {
        min_dt = step_size.dt.get().min(min_dt);
    }
    if min_dt == f32::MAX {
        return;
    }
    for mut canvas in canvases.iter_mut() {
        let first_time = !canvas.has_trajectory();
        let scenario = canvas.get_scenario(&scenarios).unwrap();
        let todo = "should be recalculated only when something changed";
        let trajectory = scenario.calculate_trajectory(min_dt);
        canvas.set_trajectory(trajectory);
        if first_time {
            canvas.auto_focus();
        }
        canvas.draw_trajectory(ui_state.strokes.trajectory);
    }
}
