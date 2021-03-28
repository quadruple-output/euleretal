use crate::prelude::*;

use super::Gather;

pub struct Kind;
pub mod comp {
    pub type UserLabel = super::UserLabel;
    pub type Duration = super::Duration;
    pub type Color = super::egui::color::Hsva;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy_ecs::Entity);

pub type Query<'a> = (
    bevy_ecs::Entity,
    &'a Kind,
    &'a comp::UserLabel,
    &'a comp::Duration,
    &'a comp::Color,
);

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

impl<'a> Gather<'a> for Query<'a> {
    type T = Gathered<'a>;

    fn gather_from(&self, _world: &'a World) -> Self::T {
        let id: bevy_ecs::Entity = self.0;
        let label: &comp::UserLabel = self.2;
        let duration: &comp::Duration = self.3;
        let color: &comp::Color = self.4;
        Gathered {
            id,
            label: &label.0,
            duration: duration.0.copy_read_only(),
            color: *color,
        }
    }
}

impl<'a> Gather<'a> for self::Entity {
    type T = Gathered<'a>;

    fn gather_from(&self, world: &'a World) -> Self::T {
        Gathered {
            id: self.0,
            label: &world.get::<comp::UserLabel>(self.0).unwrap().0,
            duration: world
                .get::<comp::Duration>(self.0)
                .unwrap()
                .0
                .copy_read_only(),
            color: *world.get::<comp::Color>(self.0).unwrap(),
        }
    }
}

pub struct Gathered<'a> {
    pub id: bevy_ecs::Entity,
    pub label: &'a String,
    pub duration: ChangeTracker<R32, change_tracker::Read>,
    pub color: Hsva,
}

impl std::fmt::Display for Gathered<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.label.is_empty() {
            write!(f, "{}", self.duration.get())
        } else {
            write!(f, "{} \"{}\"", self.duration.get(), self.label)
        }
    }
}
