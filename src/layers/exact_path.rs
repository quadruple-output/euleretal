use crate::{Canvas, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_exact_path.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
pub fn render_exact_path(
    ui_state: ResMut<UIState>,
    mut canvases: Query<&mut Canvas>, // always request canvases with 'mut'
) {
    for mut canvas in canvases.iter_mut() {
        canvas.draw_trajectory(ui_state.strokes.trajectory);
    }
}
