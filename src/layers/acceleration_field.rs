use bevy::prelude::*;
use egui::{Rgba, Stroke};

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
    let accel_stroke = Stroke::new(1., Rgba::from_rgb(0.3, 0.3, 0.8));
    scenarios.iter().for_each(|scenario| {
        let min = canvas.min();
        let max = canvas.max();
        for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
                let pos = Vec3::new(x as f32, y as f32, 0.);
                let a = scenario.acceleration().value_at(pos);
                canvas.vector(pos, pos + a, accel_stroke)
            }
        }
        canvas.on_hover_ui(|ui, mouse_pos| {
            let a = scenario.acceleration().value_at(mouse_pos);
            ui.label(format!("a.x: {:.3}", a.x));
            ui.label(format!("a.y: {:.3}", a.y));
            if a.z != 0. {
                ui.label(format!("a.z: {:.3}", a.z));
            }
            ui.label(format!("|a|: {:.3}", a.length()));
            canvas.vector(mouse_pos, mouse_pos + a, accel_stroke)
        })
    });
}
