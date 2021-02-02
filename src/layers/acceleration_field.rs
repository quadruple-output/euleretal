use bevy::prelude::*;

use crate::{scenarios::Scenario, ui::UIState};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_layer.system());
    }
}

pub fn render_layer(
    // UIState must be requested as Mut, or else it panics when other systems use it in parallel
    ui_state: ResMut<UIState>,
    scenarios: Query<&Scenario>,
) {
    //pub fn render_layer(ui_state: ResMut<UIState>) {
    if !ui_state.layerflags.acceleration_field {
        return;
    }

    let canvas = &ui_state.canvas;
    scenarios.iter().for_each(|scenario| {
        let min = canvas.min();
        let max = canvas.max();
        for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
                let pos = Vec3::new(x as f32, y as f32, 0.);
                let a_dt = scenario.acceleration().value_at(pos) * scenario.step_duration();
                canvas.vector(pos, a_dt, ui_state.strokes.acceleration)
            }
        }

        canvas.on_hover_ui(|ui, mouse_pos| {
            let a_dt = scenario.acceleration().value_at(mouse_pos) * scenario.step_duration();
            // ui.label(format!("dt*a.x: {:.3}", a_dt.x));
            // ui.label(format!("dt*a.y: {:.3}", a_dt.y));
            // if a_dt.z != 0. {
            //     ui.label(format!("dt*a.z: {:.3}", a_dt.z));
            // }
            ui.label(format!("dt*|a|: {:.3}", a_dt.length()));
            canvas.vector(mouse_pos, a_dt, ui_state.strokes.acceleration)
        })
    });
}
