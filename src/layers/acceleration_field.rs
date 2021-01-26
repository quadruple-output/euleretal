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
    if !ui_state.layerflags.acceleration_field || ui_state.canvas.is_none() {
        return;
    }

    let canvas = ui_state.canvas.as_ref().unwrap();
    let accel_stroke = Stroke::new(1., Rgba::from_rgb(0.3, 0.3, 0.8));
    scenarios.iter().for_each(|scenario| {
        canvas.on_hover_ui(|ui, mouse_pos| {
            if let Some(a) = scenario.acceleration.value_at(mouse_pos) {
                ui.label(format!("a.x: {:.3}", a.x));
                ui.label(format!("a.y: {:.3}", a.y));
                if a.z != 0. {
                    ui.label(format!("a.z: {:.3}", a.z));
                }
                ui.label(format!("|a|: {:.3}", a.length()));
                canvas.line_segment(mouse_pos, mouse_pos + a, accel_stroke)
            }
        })
    });
}
