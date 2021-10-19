mod app;
mod containers;
mod entities;
mod misc;
mod world;

use super::{core, import, integrators, scenarios, ui_import};
use world::World;

mod constants {
    pub const BUTTON_GLYPH_ADD: &str = "\u{271a}"; // \u{271a} = '✚'
    pub const BUTTON_GLYPH_DELETE: &str = "\u{2796}"; // \u{2796}='➖', \u{1fsd1} = '🗑'
}

pub use app::Euleretal;
