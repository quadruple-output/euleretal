use crate::{core::integrator::StartCondition, prelude::*};

pub struct Kind;

pub mod comp {
    use std::sync::Mutex;

    pub type State = Mutex<super::State>;
    pub type StepSizeId = super::step_size::Entity;
    pub type CanvasId = super::canvas::Entity;
    pub type IntegratorId = super::integrator::Entity;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

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
    samples: Option<Samples>,
    samples_change_count: ChangeCount,
    reference_samples: Option<Samples>,
    ref_samples_change_count: ChangeCount,
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

    pub fn reset(&mut self) {
        *self = Self::new();
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
        assert!(!points.is_empty());
        points
            .iter()
            .map(|pos| (*pos - search_pos).length_squared())
            .enumerate()
            .fold(
                (0, f32::MAX),
                |(idx0, d0), (idx1, d1)| {
                    if d0 < d1 {
                        (idx0, d0)
                    } else {
                        (idx1, d1)
                    }
                },
            )
    }
}

impl<'a> Gathered<'a> {
    pub fn reset(&self) {
        self.state.lock().unwrap().reset();
    }

    pub fn update(
        &self,
        acceleration: &dyn AccelerationField,
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
            state.samples = Some(integrator.execute(
                acceleration,
                &StartCondition {
                    s: start_position.get(),
                    v: start_velocity.get(),
                    a: acceleration.value_at(start_position.get()),
                },
                duration.get(),
                step_size.get(),
            ));
            state.samples_change_count = samples_change_count;
            if state.ref_samples_change_count != ref_samples_change_count {
                state.reference_samples = Some(scenario::calculate_reference_samples(
                    acceleration,
                    start_position.get(),
                    start_velocity.get(),
                    duration.get(),
                    step_size.get(),
                ));
                state.ref_samples_change_count = ref_samples_change_count;
            }
        }
    }

    pub fn stretch_bbox(&self, bbox: &mut BoundingBox) {
        let state = self.state.lock().unwrap();
        for samples in state.reference_samples.iter().chain(state.samples.iter()) {
            samples
                .step_points()
                .iter()
                .for_each(|&point| bbox.expand_to(point));
        }
    }

    pub fn draw_on(
        &self,
        canvas: &canvas::State,
        sample_color: Color32,
        stroke: Stroke,
        painter: &egui::Painter,
    ) {
        let state = self.state.lock().unwrap();
        if let Some(ref samples) = state.samples {
            canvas.draw_sample_trajectory(&samples, stroke, painter);
        }
        for samples in state.reference_samples.iter().chain(state.samples.iter()) {
            canvas.draw_sample_dots(samples, sample_color, painter);
        }
    }
}
