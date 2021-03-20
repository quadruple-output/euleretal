use crate::app::prelude::*;

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
    pub fn spawn(self, commands: &mut bevy_ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}
