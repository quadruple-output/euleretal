use crate::prelude::*;

pub fn show(ui: &mut Ui, state: &mut ControlState) {
    ui.heading("Colors");
    ui.vertical(|ui| {
        my_stroke_ui(ui, &mut state.strokes.trajectory, "Exact Trajectory", "");
        my_stroke_ui(
            ui,
            &mut state.strokes.acceleration,
            "Acceleration (Field)",
            "",
        );
        my_stroke_ui(ui, &mut state.strokes.coordinates, "Coordinates", "");
        my_stroke_ui(
            ui,
            &mut state.strokes.focussed_acceleration,
            "Acceleration",
            "",
        );
        my_stroke_ui(ui, &mut state.strokes.focussed_velocity, "Velocity", "");
    });
}
