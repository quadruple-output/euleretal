use crate::prelude::*;

pub struct Kind;
pub mod comp {
    pub type Integrator = Box<dyn super::Integrator>;
    pub type Stroke = eframe::egui::Stroke;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

#[derive(bevy_ecs::Bundle)]
pub struct Bundle(pub Kind, pub comp::Integrator, pub comp::Stroke);

impl Bundle {
    pub fn spawn(self, commands: &mut bevy_ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}
