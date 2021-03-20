use crate::app::prelude::*;

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: R32) -> Sample;

    fn integrate(
        &self,
        acceleration: &dyn Acceleration,
        start_position: &StartPosition,
        start_velocity: &StartVelocity,
        duration: &Duration,
        dt: R32,
    ) -> Vec<Sample> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (duration.0.get() / dt).into_inner() as usize;
        let mut result = Vec::with_capacity(num_steps + 1);
        let mut sample = Sample {
            n: 0,
            t: 0_f32.into(),
            dt,
            s: start_position.0.get(),
            v: start_velocity.0.get(),
            a: acceleration.value_at(start_position.0.get()),
        };
        result.push(sample);
        for _ in 1..=num_steps {
            sample = self.integrate_step(acceleration, sample, dt);
            result.push(sample);
        }
        result
    }
}
