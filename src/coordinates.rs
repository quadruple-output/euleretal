use crate::canvas::Canvas;
use bevy::prelude::*;
use egui::{Rgba, Stroke};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(display_coordinates.system());
    }
}

fn display_coordinates(canvas: Res<Canvas>) {
    let coord_stroke = Stroke::new(1., Rgba::from_rgb(0., 0.5, 0.) * 0.3);

    canvas.hline(0., coord_stroke);
    canvas.vline(0., coord_stroke);
    let min = canvas.min();
    let max = canvas.max();
    for step in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
        canvas.line_segment(
            Vec3::new(step as f32, -0.05, 0.),
            Vec3::new(step as f32, 0.05, 0.),
            coord_stroke,
        );
    }
    for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
        canvas.line_segment(
            Vec3::new(-0.05, step as f32, 1.),
            Vec3::new(0.05, step as f32, 1.),
            coord_stroke,
        );
    }
    canvas.on_hover_ui(|ui, mouse_pos| {
        ui.label(format!("X: {:+.4}", mouse_pos.x));
        ui.label(format!("Y: {:+.4}", mouse_pos.y));
        if mouse_pos.z != 0. {
            ui.label(format!("Y: {:+.4}", mouse_pos.z));
        }
    });
}
