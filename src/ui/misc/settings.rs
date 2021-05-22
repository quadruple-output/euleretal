use eframe::egui::{Color32, Rgba, Stroke};
use std::fmt;

pub struct ControlState {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub format_precision: usize,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            layerflags: LayerFlags::default(),
            strokes: Strokes::default(),
            format_precision: 3,
        }
    }
}

impl ControlState {
    pub fn format_f32(&self, n: f32) -> FormatterF32 {
        FormatterF32 {
            precision: self.format_precision,
            n,
        }
    }
}

pub struct FormatterF32 {
    precision: usize,
    n: f32,
}

impl fmt::Display for FormatterF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.*}", self.precision, self.n)
    }
}

pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
    pub inspector: bool,
}

impl Default for LayerFlags {
    fn default() -> Self {
        Self {
            coordinates: true,
            acceleration_field: false,
            inspector: true,
        }
    }
}

pub struct Strokes {
    pub trajectory: Stroke,
    pub acceleration: Stroke,
    pub coordinates: Stroke,
    pub focussed_velocity: Stroke,
    pub focussed_acceleration: Stroke,
}

impl Default for Strokes {
    fn default() -> Self {
        let col_accel = Rgba::from_rgb(0.3, 0.3, 0.8);
        let col_velo = Rgba::from(Color32::WHITE);
        Self {
            trajectory: Stroke::new(1., col_velo * 0.25),
            focussed_velocity: Stroke::new(1., col_velo * 1.),
            acceleration: Stroke::new(1., col_accel * 0.25),
            focussed_acceleration: Stroke::new(1., col_accel * 1.),
            coordinates: Stroke::new(1., Rgba::from_rgb(0., 0.5, 0.) * 0.3),
        }
    }
}
