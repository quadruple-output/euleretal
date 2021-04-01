use crate::prelude::*;

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate_step(&self, a: &dyn AccelerationField, sample: Sample, dt: R32) -> Sample;

    fn integrate(
        &self,
        acceleration: &dyn AccelerationField,
        start_position: Vec3,
        start_velocity: Vec3,
        duration: R32,
        dt: R32,
    ) -> Vec<Sample> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (duration / dt).into_inner() as usize;
        let mut result = Vec::with_capacity(num_steps + 1);
        let mut sample = Sample {
            n: 0,
            t: 0_f32.into(),
            dt,
            s: start_position,
            v: start_velocity,
            a: acceleration.value_at(start_position),
        };
        result.push(sample);
        for _ in 1..=num_steps {
            sample = self.integrate_step(acceleration, sample, dt);
            result.push(sample);
        }
        result
    }
}
