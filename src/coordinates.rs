use crate::canvas::Canvas;
use bevy::prelude::*;
use egui::{Pos2, Rgba, Stroke};

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
            Pos2::new(step as f32, -0.05),
            Pos2::new(step as f32, 0.05),
            coord_stroke,
        );
    }
    for step in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
        canvas.line_segment(
            Pos2::new(-0.05, step as f32),
            Pos2::new(0.05, step as f32),
            coord_stroke,
        );
    }
}
