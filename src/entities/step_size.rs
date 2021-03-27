use crate::prelude::*;

pub struct Kind;
pub mod comp {
    pub type UserLabel = super::UserLabel;
    pub type Duration = super::Duration;
    pub type Color = super::egui::color::Hsva;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

#[derive(bevy_ecs::Bundle)]
pub struct Bundle(
    pub Kind,
    pub comp::UserLabel,
    pub comp::Duration,
    pub comp::Color,
);

impl Bundle {
    pub fn spawn(self, world: &mut bevy_ecs::World) -> self::Entity {
        Entity(world.spawn(self))
    }
}
