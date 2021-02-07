use crate::{Acceleration, Sample, Scenario};
pub use euler::*;

mod euler;

pub struct ConfiguredIntegrator {
    pub integrator: Box<dyn Integrator>,
}

impl ConfiguredIntegrator {
    pub fn integrate(&self, scenario: &Scenario, dt: f32) -> Vec<Sample> {
        self.integrator.integrate(scenario, dt)
    }
}

pub trait Integrator: Send + Sync {
    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: f32) -> Sample;

    fn integrate(&self, scenario: &Scenario, dt: f32) -> Vec<Sample> {
        let num_steps = (scenario.duration() / dt) as usize;
        let mut result = Vec::with_capacity(num_steps + 1);
        let mut sample = Sample {
            n: 0,
            t: 0.,
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
