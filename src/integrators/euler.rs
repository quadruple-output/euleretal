use super::Integrator;
use crate::Sample;

#[derive(Debug)]
pub struct ImplicitEuler;

impl Integrator for ImplicitEuler {
    fn integrate<A: crate::acceleration::Acceleration>(a: A, s0: Sample, dt: f32) -> Sample {
        let _v1 = s0.v + s0.a * dt;
        let _s1 = s0.s + _v1 * dt;
        Sample {
            n: Some(s0.n.unwrap() + 1),
            t: s0.t + dt,
            dt,
            s: _s1,
            v: _v1,
            a: a.value_at(_s1),
        }
    }
}
