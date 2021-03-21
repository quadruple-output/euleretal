use super::State;
use crate::egui::{stroke_ui, Ui};

pub fn show(ui: &mut Ui, state: &mut State) {
    ui.heading("Colors");
    ui.vertical(|ui| {
        stroke_ui(ui, &mut state.strokes.trajectory, "Exact Trajectory");
        stroke_ui(ui, &mut state.strokes.acceleration, "Acceleration (Field)");
        stroke_ui(ui, &mut state.strokes.coordinates, "Coordinates");
        stroke_ui(ui, &mut state.strokes.focussed_acceleration, "Acceleration");
        stroke_ui(ui, &mut state.strokes.focussed_velocity, "Velocity");
    });
}
