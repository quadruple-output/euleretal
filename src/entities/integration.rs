use crate::prelude::*;

pub struct Kind;

pub mod comp {
    use std::sync::Mutex;

    pub type State = Mutex<super::State>;
    pub type StepSizeId = super::step_size::Entity;
    pub type CanvasId = super::canvas::Entity;
    pub type IntegratorId = super::integrator::Entity;
}

#[derive(Clone, Copy)]
pub struct Entity(bevy_ecs::Entity);

#[derive(bevy_ecs::Bundle)]
pub struct Bundle(
    pub Kind,
    pub comp::State,
    pub comp::IntegratorId,
    pub comp::StepSizeId,
    pub comp::CanvasId,
);

pub type Query<'a> = (
    bevy_ecs::Entity,
    &'a Kind,
    &'a comp::State,
    &'a comp::IntegratorId,
    &'a comp::StepSizeId,
    &'a comp::CanvasId,
);

pub struct Gathered<'a> {
    pub id: bevy_ecs::Entity,
    pub state: &'a comp::State,
    pub integrator_id: comp::IntegratorId,
    pub integrator: &'a dyn Integrator,
    pub stroke: &'a integrator::comp::Stroke,
    pub step_size_id: comp::StepSizeId,
    pub step_duration: ChangeTracker<R32, change_tracker::Read>,
    pub step_label: &'a String,
    pub step_color: step_size::comp::Color,
    pub canvas_id: bevy_ecs::Entity,
}

impl Bundle {
    pub fn spawn(self, world: &mut bevy_ecs::World) -> self::Entity {
        Entity(world.spawn(self))
    }
}

impl<'a> super::Gather<'a> for Query<'a> {
    type T = Gathered<'a>;
    fn gather_from(&self, world: &'a World) -> Gathered<'a> {
        // enforce type check for assignments:
        let id: bevy_ecs::Entity = self.0;
        let state: &comp::State = self.2;
        let integrator_id: &comp::IntegratorId = self.3;
        let step_size_id: &comp::StepSizeId = self.4;
        let canvas_id: &comp::CanvasId = self.5;
        let integrator = world
            .get::<integrator::comp::Integrator>(integrator_id.0)
            .unwrap();
        let stroke = world
            .get::<integrator::comp::Stroke>(integrator_id.0)
            .unwrap();
        let step_duration = world
            .get::<step_size::comp::Duration>(step_size_id.0)
            .unwrap();
        let step_label = world
            .get::<step_size::comp::UserLabel>(step_size_id.0)
            .unwrap();
        let step_color = world.get::<step_size::comp::Color>(step_size_id.0).unwrap();
        Gathered {
            id,
            state,
            integrator_id: *integrator_id,
            integrator: &**integrator,
            stroke,
            step_size_id: *step_size_id,
            step_duration: step_duration.0.copy_read_only(),
            step_label: &step_label.0,
            step_color: *step_color,
            canvas_id: canvas_id.0,
        }
    }
}

pub struct State {
    samples: Vec<Sample>,
    samples_change_count: ChangeCount,
    reference_samples: Vec<Sample>,
    ref_samples_change_count: ChangeCount,
}

impl State {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            samples_change_count: ChangeCount::default(),
            reference_samples: Vec::new(),
            ref_samples_change_count: ChangeCount::default(),
        }
    }

    /// returns (ReferenceSample,ComputedSample)
    pub fn closest_sample(&self, pos: Vec3) -> Option<(Sample, Sample)> {
        if self.reference_samples.is_empty() {
            None
        } else {
            let (closest_reference, dist_ref) = Self::find_closest(&self.reference_samples, pos);
            let (closest_sample, dist_sample) = Self::find_closest(&self.samples, pos);
            if dist_ref < dist_sample {
                Some((closest_reference, self.samples[closest_reference.n]))
            } else {
                Some((self.reference_samples[closest_sample.n], closest_sample))
            }
        }
    }

    fn find_closest(samples: &[Sample], pos: Vec3) -> (Sample, f32) {
        assert!(!samples.is_empty());
        samples
            .iter()
            .map(|s| (s, (s.s - pos).length_squared()))
            .fold((Sample::default(), f32::MAX), |(s0, d0), (&s1, d1)| {
                if d0 < d1 {
                    (s0, d0)
                } else {
                    (s1, d1)
                }
            })
    }
}

impl<'a> Gathered<'a> {
    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        *state = State::new();
    }

    pub fn update(
        &self,
        acceleration: &dyn Acceleration,
        start_position: &ChangeTracker<Vec3, impl change_tracker::TRead>,
        start_velocity: &ChangeTracker<Vec3, impl change_tracker::TRead>,
        duration: &ChangeTracker<R32, impl change_tracker::TRead>,
        integrator: &dyn Integrator,
        step_size: &ChangeTracker<R32, impl change_tracker::TRead>,
    ) {
        let mut state = self.state.lock().unwrap();
        let ref_samples_change_count = step_size.change_count()
            + start_position.change_count()
            + start_velocity.change_count()
            + duration.change_count();
        let samples_change_count = ref_samples_change_count; // + integrator.change_count();
        if state.samples_change_count != samples_change_count {
            state.samples = integrator.integrate(
                acceleration,
                start_position.get(),
                start_velocity.get(),
                duration.get(),
                step_size.get(),
            );
            state.samples_change_count = samples_change_count;
            if state.ref_samples_change_count != ref_samples_change_count {
                state.reference_samples = scenario::calculate_reference_samples(
                    acceleration,
                    start_position.get(),
                    start_velocity.get(),
                    duration.get(),
                    step_size.get(),
                );
                state.ref_samples_change_count = ref_samples_change_count;
            }
        }
    }

    pub fn stretch_bbox(&self, bbox: &mut BoundingBox) {
        let state = self.state.lock().unwrap();
        state
            .reference_samples
            .iter()
            .for_each(|&sample| bbox.expand_to(sample.s));
        state
            .samples
            .iter()
            .for_each(|&sample| bbox.expand_to(sample.s));
    }

    pub fn draw_on(
        &self,
        canvas: &canvas::State,
        sample_color: Color32,
        stroke: Stroke,
        painter: &egui::Painter,
    ) {
        let state = self.state.lock().unwrap();
        canvas.draw_sample_trajectory(&state.samples, stroke, painter);
        canvas.draw_sample_dots(&state.reference_samples, sample_color, painter);
        canvas.draw_sample_dots(&state.samples, sample_color, painter);
    }
}
