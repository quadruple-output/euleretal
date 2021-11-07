use super::{
    integration_step, AccelerationField, Duration, Integrator, Position, Samples, Scenario,
    StartCondition,
};
use ::std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct Integration {
    /// invariant: samples.len() == reference_samples.len()
    samples: Option<Samples>,
    sample_validity: u64,
    /// invariant: samples.len() == reference_samples.len()
    reference_samples: Option<Samples>,
    ref_sample_validity: u64,
}

impl Default for Integration {
    fn default() -> Self {
        Self::new()
    }
}

impl Integration {
    #[must_use]
    pub fn new() -> Self {
        Self {
            samples: None,
            sample_validity: 0,
            reference_samples: None,
            ref_sample_validity: 0,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    /// returns `true` if something was actually updated
    pub fn update(
        &mut self,
        scenario: &Scenario,
        integrator: &dyn Integrator,
        step_duration: Duration,
    ) -> bool {
        // check if we have to re-calculate:
        let mut hasher = DefaultHasher::new();
        scenario.hash_default(&mut hasher);
        step_duration.hash(&mut hasher);
        let ref_sample_validity = hasher.finish();
        integrator.hash(&mut hasher);
        let sample_validity = hasher.finish();

        if sample_validity == self.sample_validity {
            false
        } else {
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            let num_steps = (scenario.duration / step_duration) as usize;

            let samples = Self::integrate(
                integrator,
                &*scenario.acceleration,
                &StartCondition::new(
                    scenario.start_position,
                    scenario.start_velocity,
                    scenario.acceleration.value_at(scenario.start_position),
                ),
                num_steps,
                step_duration,
            );
            let num_samples = samples.len();
            assert!(num_samples == num_steps);
            self.samples = Some(samples);
            self.sample_validity = sample_validity;

            if self.ref_sample_validity != ref_sample_validity {
                let reference_samples = scenario.calculate_reference_samples(step_duration);
                let num_refs = reference_samples.len();
                assert!(num_refs == num_samples);
                self.reference_samples = Some(reference_samples);
                self.ref_sample_validity = ref_sample_validity;
            }
            true
        }
    }

    fn integrate(
        integrator: &dyn Integrator,
        acceleration_field: &dyn AccelerationField,
        start_condition: &StartCondition,
        num_steps: usize,
        dt: Duration,
    ) -> Samples {
        let start = ::std::time::Instant::now();

        let mut samples = Samples::new(num_steps);

        let mut step = integration_step::Step::new(start_condition, dt);
        let mut builder = integration_step::builders::Step::new(acceleration_field, &mut step);
        for _ in 0..num_steps {
            let ((s, v, a), dt) = (builder.start_values(), builder.dt());
            integrator.integrate_step(s, v, a, dt, &mut builder);
            builder.finalize();
            let next_step = step.create_next();
            samples.push_sample(step);
            step = next_step;
            builder = integration_step::builders::Step::new(acceleration_field, &mut step);
        }
        let result = samples.finalized();

        log::info!("{}: {}µs", integrator.label(), start.elapsed().as_micros());
        result
    }

    /// invariant: `samples()?.len() == reference_samples()?.len()`
    #[must_use]
    pub fn reference_samples(&self) -> Option<&Samples> {
        self.reference_samples.as_ref()
    }

    /// invariant: `samples()?.len() == reference_samples()?.len()`
    #[must_use]
    pub fn samples(&self) -> Option<&Samples> {
        self.samples.as_ref()
    }

    /// Finds the (computed or reference) sample which is closest to the given pointer position.
    /// Returns `None` if there are no samples.
    #[must_use]
    pub fn closest_sample_index(&self, pos: &Position) -> Option<usize> {
        if let (Some(references), Some(samples)) =
            (self.reference_samples.as_ref(), self.samples.as_ref())
        {
            if let (Some(closest_reference), Some(closest_sample)) =
                (references.closest(pos), samples.closest(pos))
            {
                return if closest_reference.distance < closest_sample.distance {
                    Some(closest_reference.index)
                } else {
                    Some(closest_sample.index)
                };
            }
        }
        None
    }
}
