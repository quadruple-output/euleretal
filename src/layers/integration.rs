use crate::{Canvas, Integration, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(render_integration.system());
    }
}

pub fn render_integration(
    ui_state: Res<UIState>,
    integrations: Query<&Integration>,
    mut canvases: Query<&mut Canvas>,
) {
    for integration in integrations.iter() {
        let canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();
        integration.draw_on(&canvas, ui_state.colors.exact_sample.into());
    }
}
