use super::core::PhysicalQuantityKind;

use super::ui_import::{egui::Painter, Color32, Pos2, Rgba, Stroke, Vec2};
use ::std::fmt;

#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct Settings {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub point_formats: PointFormats,
    pub format_precision: usize,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
    pub inspector: bool,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct Strokes {
    pub trajectory: Stroke,
    pub acceleration: Stroke,
    pub coordinates: Stroke,
    pub focussed_velocity: Stroke,
    pub focussed_acceleration: Stroke,
    /// to be used for velocities that are the basis for a derived velocity
    pub start_velocity: Stroke,
    /// to be used for velocities that contribute to a derived position
    pub contributing_velocity: Stroke,
    /// to be used for acceleration that contribute to a derived position or velocity
    pub contributing_acceleration: Stroke,
    pub derived_velocity: Stroke,
    pub reference_velocity: Stroke,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct PointFormats {
    /// to be used for positions that are the basis for a derived position
    pub start_position: PointFormat,
    pub derived_position: PointFormat,
    pub reference_position: PointFormat,
    pub other_position: PointFormat,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct PointFormat {
    pub shape: PointShape,
    // size of the shape in screen dimensions
    pub size: f32,
    // for circles, only the stroke's color is considered
    pub stroke: Stroke,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub enum PointShape {
    Dot,
    CrossHair,
}

impl Default for LayerFlags {
    fn default() -> Self {
        Self {
            coordinates: true,
            acceleration_field: false,
            inspector: true,
        }
    }
}

impl Default for PointFormats {
    fn default() -> Self {
        Self {
            start_position: PointFormat {
                shape: PointShape::Dot,
                size: 5.,
                stroke: Stroke::new(1., Color32::RED),
            },
            derived_position: PointFormat {
                shape: PointShape::Dot,
                size: 5.,
                stroke: Stroke::new(1., Color32::GREEN),
            },
            reference_position: PointFormat {
                shape: PointShape::CrossHair,
                size: 8.,
                stroke: Stroke::new(1., Color32::GREEN),
            },
            other_position: PointFormat {
                shape: PointShape::Dot,
                size: 5.,
                stroke: Stroke::new(1., Color32::GRAY),
            },
        }
    }
}

impl Default for Strokes {
    fn default() -> Self {
        let col_accel = Rgba::from_rgb(0.3, 0.3, 0.8);
        let col_velo = Rgba::from(Color32::WHITE);
        Self {
            trajectory: Stroke::new(1., col_velo * 0.25),
            focussed_velocity: Stroke::new(1., col_velo * 1.),
            acceleration: Stroke::new(1., col_accel * 0.25),
            focussed_acceleration: Stroke::new(1., col_accel * 1.),
            coordinates: Stroke::new(1., Rgba::from_rgb(0., 0.5, 0.) * 0.3),
            start_velocity: Stroke::new(1., PointFormats::default().start_position.stroke.color),
            contributing_velocity: Stroke::new(1., col_velo),
            contributing_acceleration: Stroke::new(1., col_accel),
            derived_velocity: PointFormats::default().derived_position.stroke,
            reference_velocity: PointFormats::default().reference_position.stroke,
        }
    }
}

impl Strokes {
    /// # Panics
    /// Panics if kind is `_::Position`
    pub fn for_contribution(&self, kind: PhysicalQuantityKind) -> Stroke {
        match kind {
            PhysicalQuantityKind::Position => panic!(),
            PhysicalQuantityKind::Velocity => self.contributing_velocity,
            PhysicalQuantityKind::Acceleration => self.contributing_acceleration,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            layerflags: LayerFlags::default(),
            strokes: Strokes::default(),
            point_formats: PointFormats::default(),
            format_precision: 3,
        }
    }
}

impl Settings {
    pub fn format_f32(&self, n: f32) -> FormatterF32 {
        FormatterF32 {
            precision: self.format_precision,
            n,
        }
    }
}

pub struct FormatterF32 {
    precision: usize,
    n: f32,
}

impl fmt::Display for FormatterF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.*}", self.precision, self.n)
    }
}

impl PointFormat {
    pub fn draw_position_on(&self, position: Pos2, painter: &Painter) {
        let radius = self.size * 0.5;
        match self.shape {
            PointShape::Dot => {
                painter.circle_filled(position, radius, self.stroke.color);
            }
            PointShape::CrossHair => {
                let x_radius = Vec2::new(radius, 0.);
                let y_radius = Vec2::new(0., radius);
                painter.line_segment([position - x_radius, position + x_radius], self.stroke);
                painter.line_segment([position - y_radius, position + y_radius], self.stroke);
            }
        }
    }
}
