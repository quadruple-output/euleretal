use super::{
    import::Vec3, CompleteSample, Duration, FinalizedCalibrationPoints, Integrator, Position,
    Samples, Scenario, StartCondition,
};
use ::std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct Integration {
    samples: Option<Samples>,
    sample_validity: u64,
    reference_samples: Option<Samples>,
    scenario_hash: u64,
}

impl Integration {
    pub fn new() -> Self {
        Self {
            samples: None,
            sample_validity: 0,
            reference_samples: None,
            scenario_hash: 0,
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
        let scenario_hash = hasher.finish();
        integrator.hash(&mut hasher);
        step_duration.0.hash(&mut hasher);
        let sample_validity = hasher.finish();

        if self.sample_validity != sample_validity {
            #[allow(clippy::cast_sign_loss)]
            let num_steps = (scenario.duration.0 / step_duration.0).into_inner() as usize;

            let samples = integrator.integrate(
                &*scenario.acceleration,
                &StartCondition {
                    position: scenario.start_position.0,
                    velocity: scenario.start_velocity.0,
                    acceleration: scenario.acceleration.value_at(scenario.start_position.0),
                },
                num_steps,
                step_duration.0,
            );
            let num_samples = samples.step_points().len();
            assert!(num_samples == num_steps + 1);
            self.samples = Some(samples);
            self.sample_validity = sample_validity;

            if self.scenario_hash != scenario_hash {
                let reference_samples = scenario.calculate_reference_samples(step_duration.0);
                let num_refs = reference_samples.step_points().len();
                assert!(num_refs == num_samples);
                self.reference_samples = Some(reference_samples);
                self.scenario_hash = scenario_hash;
            }
        }
    }

    pub fn reference_samples(&self) -> Option<&Samples<FinalizedCalibrationPoints>> {
        self.reference_samples.as_ref()
    }

    pub fn samples(&self) -> Option<&Samples<FinalizedCalibrationPoints>> {
        self.samples.as_ref()
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn closest_sample(&self, pos: Vec3) -> Option<(CompleteSample, CompleteSample)> {
        self.reference_samples.as_ref().and_then(|references| {
            self.samples.as_ref().map(|samples| {
                let (idx_reference, dist_reference) =
                    Self::find_closest(&references.step_points(), pos);
                let (idx_sample, dist_sample) = Self::find_closest(&samples.step_points(), pos);
                if dist_reference < dist_sample {
                    (references.at(idx_reference), samples.at(idx_reference))
                } else {
                    (references.at(idx_sample), samples.at(idx_sample))
                }
            })
        })
    }

    fn find_closest(points: &[Position], search_pos: Position) -> (usize, f32) {
        points
            .iter()
            .map(|pos| (*pos - search_pos).length_squared())
            .enumerate()
            .reduce(|closest_so_far, current| {
                if closest_so_far.1 <= current.1 {
                    closest_so_far
                } else {
                    current
                }
            })
            .unwrap()
    }
}
