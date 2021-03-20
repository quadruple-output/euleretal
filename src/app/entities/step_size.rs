pub struct Kind;
pub mod comp {
    pub type UserLabel = crate::UserLabel;
    pub type Duration = crate::Duration;
    pub type Color = egui::color::Hsva;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::prelude::Entity);

#[derive(bevy::ecs::Bundle)]
pub struct Bundle(
    pub Kind,
    pub comp::UserLabel,
    pub comp::Duration,
    pub comp::Color,
);

impl Bundle {
    pub fn spawn(self, commands: &mut bevy::ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}
