use bevy::math::Vec3;
use egui::{Color32, Stroke};

use crate::{acceleration::Acceleration, canvas::Canvas};

pub mod center_mass;

pub struct Scenario {
    pub acceleration: Box<dyn Acceleration>,
    pub start_position: Vec3,
    pub start_velocity: Vec3,
    /// delta t (single time step duration)
    pub dt: f32,
    /// simulation duration (for drawing exact solution)
    pub draw_t: f32,
}

impl Scenario {
    pub fn draw_on(&self, canvas: &Canvas) {
        const STEPS_PER_DT: usize = 1000;
        let stroke = Stroke::new(1., Color32::WHITE);
        let h = self.dt / STEPS_PER_DT as f32;

        let mut s0 = self.start_position;
        let mut v0 = self.start_velocity;
        let mut t = 0.;
        while t < self.draw_t {
            let a0 = self.acceleration.value_at(s0);
            // let s1 = s0 + v0 * h + a0 * h * h;  // std. Euler.  Good for circles
            let s1 = s0 + v0 * h + 0.5 * a0 * h * h; // Exact for uniform acceleration
            let v1 = v0 + a0 * h;
            canvas.line_segment(s0, s1, stroke);
            s0 = s1;
            v0 = v1;
            t += h;
        }
    }
}
