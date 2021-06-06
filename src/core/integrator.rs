use super::{
    import::R32,
    samples::{NewSampleWithPoints, Samples, StartCondition},
    AccelerationField,
};
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait Integrator: Send + Sync + 'static {
    fn label(&self) -> String;

    fn description(&self) -> String;

    fn integrate(
        &self,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: R32,
    ) -> Samples {
        let mut samples = Samples::new(start_condition, self.num_calibration_points(), num_steps);
        for _ in 0..num_steps {
            let current = samples.current().unwrap();
            let mut next = NewSampleWithPoints {
                dt,
                ..NewSampleWithPoints::default()
            };

            self.integrate_step(&current, &mut next, dt.into(), acceleration_field);

            next.acceleration = acceleration_field.value_at(next.position);
            samples.push_sample(&next);
        }
        samples.finalized()
    }

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }

    fn num_calibration_points(&self) -> usize {
        0
    }

    fn integrate_step(
        &self,
        current: &StartCondition,
        next: &mut NewSampleWithPoints,
        dt: f32,
        acceleration_field: &dyn AccelerationField,
    );
}
