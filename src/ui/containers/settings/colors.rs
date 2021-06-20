use super::{
    misc::{my_stroke_ui, settings::Strokes},
    ui_import::Ui,
};

pub fn show(ui: &mut Ui, strokes: &mut Strokes) {
    ui.vertical(|ui| {
        my_stroke_ui(ui, &mut strokes.trajectory, "Exact Trajectory", "");
        my_stroke_ui(ui, &mut strokes.acceleration, "Acceleration (Field)", "");
        my_stroke_ui(ui, &mut strokes.coordinates, "Coordinates", "");
        my_stroke_ui(ui, &mut strokes.focussed_acceleration, "Acceleration", "");
        my_stroke_ui(ui, &mut strokes.focussed_velocity, "Velocity", "");
    });
}
