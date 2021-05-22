use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{core::samples::StartCondition, prelude::*};

pub struct Integration {
    pub integrator_conf: Obj<crate::ui::Integrator>, // todo: change to `core::Integrator` and move contained `Stroke` up to `Canvas`
    pub step_size: Obj<StepSize>,
    state: State,
}

#[derive(Default)]
pub struct State {
    samples: Option<Samples>,
    sample_validity: u64,
    reference_samples: Option<Samples>,
    scenario_hash: u64,
}

impl Integration {
    pub fn new(integrator_conf: Obj<crate::ui::Integrator>, step_size: Obj<StepSize>) -> Self {
        Self {
            integrator_conf,
            step_size,
            state: State::new(),
        }
    }

    pub fn set_integrator(&mut self, integrator_conf: Obj<crate::ui::Integrator>) {
        self.integrator_conf = integrator_conf;
        self.reset();
    }

    pub fn set_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_size = step_size;
        self.reset();
    }

    #[must_use]
    pub fn get_stroke(&self) -> Stroke {
        self.integrator_conf.borrow().stroke
    }

    #[must_use]
    pub fn get_step_color(&self) -> Hsva {
        self.step_size.borrow().color
    }

    pub fn reset(&mut self) {
        self.state = State::new();
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn update(&mut self, scenario: &Scenario) {
        let integrator = &self.integrator_conf.borrow().integrator;
        let step_duration = &self.step_size.borrow().duration;

        // check if we have to re-calculate:
        let mut hasher = DefaultHasher::new();
        scenario.hash_default(&mut hasher);
        let scenario_hash = hasher.finish();
        integrator.hash(&mut hasher);
        step_duration.0.hash(&mut hasher);
        let sample_validity = hasher.finish();

        let state = &mut self.state;
        if state.sample_validity != sample_validity {
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
            state.samples = Some(samples);
            state.sample_validity = sample_validity;

            if state.scenario_hash != scenario_hash {
                let reference_samples = scenario.calculate_reference_samples(step_duration.0);
                let num_refs = reference_samples.step_points().len();
                assert!(num_refs == num_samples);
                state.reference_samples = Some(reference_samples);
                state.scenario_hash = scenario_hash;
            }
        }
    }

    pub fn stretch_bbox(&self, bbox: &mut crate::ui::BoundingBox) {
        for samples in self
            .state
            .reference_samples
            .iter()
            .chain(self.state.samples.iter())
        {
            samples
                .step_points()
                .iter()
                .for_each(|&point| bbox.expand_to(point));
        }
    }

    pub fn draw_on(
        &self,
        canvas: &crate::ui::Canvas,
        sample_color: Color32,
        stroke: Stroke,
        painter: &egui::Painter,
    ) {
        let state = &self.state;
        if let Some(ref samples) = state.samples {
            canvas.draw_sample_trajectory(&samples, stroke, painter);
        }
        for samples in state.reference_samples.iter().chain(state.samples.iter()) {
            canvas.draw_sample_dots(samples, sample_color, painter);
        }
    }

    /// returns (ReferenceSample,ComputedSample)
    #[must_use]
    pub fn closest_sample(&self, pos: Vec3) -> Option<(CompleteSample, CompleteSample)> {
        self.state
            .reference_samples
            .as_ref()
            .and_then(|references| {
                self.state.samples.as_ref().map(|samples| {
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

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            samples: None,
            sample_validity: 0,
            reference_samples: None,
            scenario_hash: 0,
        }
    }
}
