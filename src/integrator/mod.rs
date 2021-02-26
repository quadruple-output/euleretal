use crate::{Acceleration, Sample, Scenario, TrackedChange};
use decorum::R32;
use egui::Stroke;

pub mod euler;

#[derive(bevy::ecs::Bundle)]
pub struct Bundle {
    pub integrator: Box<dyn Integrator>,
    pub stroke: Stroke,
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::ecs::Entity);

impl TrackedChange for Bundle {
    fn change_count(&self) -> crate::change_tracker::ChangeCount {
        0
    }
}

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: R32) -> Sample;

    fn integrate(&self, scenario: &Scenario, dt: R32) -> Vec<Sample> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (scenario.duration() / dt).into_inner() as usize;
        let mut result = Vec::with_capacity(num_steps + 1);
        let mut sample = Sample {
            n: 0,
            t: 0_f32.into(),
            dt,
            s: scenario.s0(),
            v: scenario.v0(),
            a: scenario.acceleration().value_at(scenario.s0()),
        };
        result.push(sample);
        for _ in 1..=num_steps {
            sample = self.integrate_step(scenario.acceleration(), sample, dt);
            result.push(sample);
        }
        result
    }
}
