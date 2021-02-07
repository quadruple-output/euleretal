use super::Integrator;
use crate::{Acceleration, Sample};

#[derive(Debug)]
pub struct ImplicitEuler;

impl Integrator for ImplicitEuler {
    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: f32) -> Sample {
        let s0 = sample.s;
        let v0 = sample.v;
        let a0 = sample.a;
        let v1 = v0 + a0 * dt;
        let s1 = s0 + v1 * dt;
        Sample {
            n: sample.n + 1,
            t: sample.t + dt,
            dt,
            s: s1,
            v: v1,
            a: a.value_at(s1),
        }
    }
}
