use bevy::prelude::*;

use crate::{Scenario, UIState};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
pub fn render(ui_state: ResMut<UIState>, scenarios: Query<&Scenario>) {
    for scenario in scenarios.iter() {
        scenario.draw_on(&ui_state.canvas);
    }
}
