use crate::{core::samples::StartCondition, prelude::*};

pub struct Integration {
    pub integrator_conf: Obj<ui::Integrator>, // todo: change to `core::Integrator` and move contained `Stroke` up to `Canvas`
    pub step_size: Obj<StepSize>,
    state: State,
}

pub struct State {
    samples: Option<Samples>,
    samples_change_count: ChangeCount,
    reference_samples: Option<Samples>,
    ref_samples_change_count: ChangeCount,
}

impl Integration {
    pub fn new(integrator_conf: Obj<ui::Integrator>, step_size: Obj<StepSize>) -> Self {
        Self {
            integrator_conf,
            step_size,
            state: State::new(),
        }
    }

    pub fn set_integrator(&mut self, integrator_conf: Obj<ui::Integrator>) {
        self.integrator_conf = integrator_conf;
        self.reset();
    }

    pub fn set_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_size = step_size;
        self.reset();
    }

    pub fn get_stroke(&self) -> Stroke {
        self.integrator_conf.borrow().stroke
    }

    pub fn get_step_color(&self) -> Hsva {
        self.step_size.borrow().color
    }

    pub fn reset(&mut self) {
        self.state = State::new();
    }

    pub fn update(
        &mut self,
        acceleration: &dyn AccelerationField,
        start_position: &StartPosition,
        start_velocity: &StartVelocity,
        duration: &Duration,
    ) {
        let step_duration = &self.step_size.borrow().duration;
        let integrator = &self.integrator_conf.borrow().integrator;
        let state = &mut self.state;
        let ref_samples_change_count = step_duration.0.change_count()
            + start_position.0.change_count()
            + start_velocity.0.change_count()
            + duration.0.change_count();
        let samples_change_count = ref_samples_change_count; // + integrator.change_count();
        if state.samples_change_count != samples_change_count {
            #[allow(clippy::cast_sign_loss)]
            let num_steps = (duration.get() / step_duration.get()).into_inner() as usize;

            state.samples = Some(integrator.integrate(
                acceleration,
                &StartCondition {
                    position: start_position.0.get(),
                    velocity: start_velocity.0.get(),
                    acceleration: acceleration.value_at(start_position.0.get()),
                },
                num_steps,
                step_duration.get(),
            ));
            state.samples_change_count = samples_change_count;
            let num_samples = state.samples.as_ref().unwrap().step_points().len();
            assert!(num_samples == num_steps + 1);

            if state.ref_samples_change_count != ref_samples_change_count {
                state.reference_samples = Some(scenario::calculate_reference_samples(
                    acceleration,
                    start_position.0.get(),
                    start_velocity.0.get(),
                    duration.get(),
                    step_duration.0.get(),
                ));
                state.ref_samples_change_count = ref_samples_change_count;
                let num_refs = state
                    .reference_samples
                    .as_ref()
                    .unwrap()
                    .step_points()
                    .len();
                assert!(num_refs == num_samples);
            }
        }
    }

    pub fn stretch_bbox(&self, bbox: &mut BoundingBox) {
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
        canvas: &Canvas,
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
    pub fn new() -> Self {
        Self {
            samples: None,
            samples_change_count: ChangeCount::default(),
            reference_samples: None,
            ref_samples_change_count: ChangeCount::default(),
        }
    }
}
