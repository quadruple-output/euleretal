use crate::app::prelude::*;

#[derive(Debug)]
pub struct Implicit;

impl Integrator for Implicit {
    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: R32) -> Sample {
        let s0 = sample.s;
        let v0 = sample.v;
        let a0 = sample.a;
        let v1 = v0 + a0 * dt.into_inner();
        let s1 = s0 + v1 * dt.into_inner();
        Sample {
            n: sample.n + 1,
            t: sample.t + dt,
            dt,
            s: s1,
            v: v1,
            a: a.value_at(s1),
        }
    }

    fn label(&self) -> String {
        "Implicit Euler".to_string()
    }
}
