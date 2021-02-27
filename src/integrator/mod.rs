use crate::{scenario, Acceleration, ChangeCount, Sample, Scenario, TrackedChange};
use decorum::R32;
use egui::Stroke;

pub mod euler;

#[derive(bevy::ecs::Bundle)]
pub struct Bundle(pub Box<dyn Integrator>, pub Stroke);
pub type Query<'a> = (&'a Box<dyn Integrator>, &'a Stroke);

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::ecs::Entity);

impl Bundle {
    pub fn spawn(self, commands: &mut bevy::ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}

impl TrackedChange for Bundle {
    fn change_count(&self) -> ChangeCount {
        0
    }
}

pub trait Integrator: Send + Sync {
    fn label(&self) -> String;

    fn integrate_step(&self, a: &dyn Acceleration, sample: Sample, dt: R32) -> Sample;

    fn integrate(&self, scenario: &scenario::Query, dt: R32) -> Vec<Sample> {
        #[allow(clippy::cast_sign_loss)]
        let num_steps = (scenario.duration().0.get() / dt).into_inner() as usize;
        let mut result = Vec::with_capacity(num_steps + 1);
        let mut sample = Sample {
            n: 0,
            t: 0_f32.into(),
            dt,
            s: scenario.start_position().0.get(),
            v: scenario.start_velocity().0.get(),
            a: scenario
                .acceleration()
                .value_at(scenario.start_position().0.get()),
        };
        result.push(sample);
        for _ in 1..=num_steps {
            sample = self.integrate_step(scenario.acceleration(), sample, dt);
            result.push(sample);
        }
        result
    }
}
