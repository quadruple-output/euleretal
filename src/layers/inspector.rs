use crate::{Canvas, Integration, UIState};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(inspector.system());
    }
}

pub fn inspector(
    ui_state: ResMut<UIState>,
    integrations: Query<&Integration>,
    mut canvases: Query<&mut Canvas>,
) {
    if !ui_state.layerflags.inspector {
        return;
    }
    for integration in integrations.iter() {
        let canvas = canvases.get_mut(integration.get_canvas_id()).unwrap();
        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some(sample) = integration.closest_sample(mouse_pos) {
                canvas.vector(
                    sample.s,
                    sample.v * sample.dt,
                    ui_state.strokes.focussed_velocity,
                );
                canvas.vector(
                    sample.s,
                    sample.a * sample.dt,
                    ui_state.strokes.focussed_acceleration,
                );

                ui.label("Inspector");
                ui.separator();
                ui.label(format!(
                    "#{}: t = {}",
                    sample.n,
                    ui_state.format_f32(sample.t)
                ));
            }
        });
    }
}
