use super::{
    import::Vec3, AccelerationField, Duration, IntegrationStep, Integrator, Samples, Scenario,
    StartCondition,
};
use ::std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct Integration {
    samples: Option<Samples>,
    sample_validity: u64,
    reference_samples: Option<Samples>,
    ref_sample_validity: u64,
}

impl Integration {
    pub fn new() -> Self {
        Self {
            samples: None,
            sample_validity: 0,
            reference_samples: None,
            ref_sample_validity: 0,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn update(
        &mut self,
        scenario: &Scenario,
        integrator: &dyn Integrator,
        step_duration: Duration,
    ) {
        // check if we have to re-calculate:
        let mut hasher = DefaultHasher::new();
        scenario.hash_default(&mut hasher);
        step_duration.0.hash(&mut hasher);
        let ref_sample_validity = hasher.finish();
        integrator.hash(&mut hasher);
        let sample_validity = hasher.finish();

        if self.sample_validity != sample_validity {
            #[allow(clippy::cast_sign_loss)]
            let num_steps = (scenario.duration.0 / step_duration.0).into_inner() as usize;

            let samples = Self::integrate(
                integrator,
                &*scenario.acceleration,
                &StartCondition {
                    position: scenario.start_position.0,
                    velocity: scenario.start_velocity.0,
                    acceleration: scenario.acceleration.value_at(scenario.start_position.0),
                },
                num_steps,
                step_duration,
            );
            let num_samples = samples.len();
            assert!(num_samples == num_steps);
            self.samples = Some(samples);
            self.sample_validity = sample_validity;

            if self.ref_sample_validity != ref_sample_validity {
                let reference_samples = scenario.calculate_reference_samples(step_duration.0);
                let num_refs = reference_samples.len();
                assert!(num_refs == num_samples);
                self.reference_samples = Some(reference_samples);
                self.ref_sample_validity = ref_sample_validity;
            }
        }
    }

    fn integrate(
        integrator: &dyn Integrator,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: Duration,
    ) -> Samples {
        let dt = dt.0;
        let mut samples = Samples::new(num_steps);
        let mut current_condition = (*start_condition).clone();
        for _ in 0..num_steps {
            let mut next =
                integrator.integrate_step(&current_condition, Duration(dt), acceleration_field);
            next.compute_acceleration_at_last_position(acceleration_field);

            current_condition = next.next_condition().unwrap();
            samples.push_sample(next);
        }
        samples.finalized()
    }

    pub fn reference_samples(&self) -> Option<&Samples> {
        self.reference_samples.as_ref()
    }

    pub fn samples(&self) -> Option<&Samples> {
        self.samples.as_ref()
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn closest_sample(&self, pos: &Vec3) -> Option<(&IntegrationStep, &IntegrationStep)> {
        if let (Some(references), Some(samples)) =
            (self.reference_samples.as_ref(), self.samples.as_ref())
        {
            if let (Some(closest_reference), Some(closest_sample)) =
                (references.closest(pos), samples.closest(pos))
            {
                if closest_reference.distance < closest_sample.distance {
                    Some((
                        references.at(closest_reference.index),
                        samples.at(closest_reference.index),
                    ))
                } else {
                    Some((
                        references.at(closest_sample.index),
                        samples.at(closest_sample.index),
                    ))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
