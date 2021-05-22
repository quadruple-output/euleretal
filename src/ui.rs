mod canvas_grid;
mod canvas_view;
mod color_controls;
mod entities;
mod integrator_controls;
mod layer_controls;
mod layers;
mod misc;
mod scenario_controls;
mod settings;
mod step_size_controls;
mod world;

use crate::{integrators, prelude::*, scenarios};
use ::core::fmt;
use eframe::{egui, epi};
use egui::{CentralPanel, CollapsingHeader, Rgba, SidePanel};
use entities::{Canvas, Integration, Integrator};
pub use misc::BoundingBox; // todo: should not be pub
pub use misc::UserLabel; // todo: should not be pub
use std::str;
use world::World;

const BUTTON_GLYPH_ADD: &str = "\u{271a}"; // \u{271a} = '✚'
const BUTTON_GLYPH_DELETE: &str = "\u{2796}"; // \u{2796}='➖', \u{1fsd1} = '🗑'
const SAMPLE_DOT_RADIUS: f32 = 2.5; // todo: this might become configurable later

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct App {
    world: World,
    //resources: Resources,
    control_state: ControlState,
}

impl Default for App {
    fn default() -> Self {
        let mut default = Self::new();
        default.initialize_scenario();
        default
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "euleretal"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn setup(&mut self, ctx: &egui::CtxRef) {
        let mut style = (*ctx.style()).clone();

        /* -=- Change Color Scheme to B/W -=- *\
        style.visuals.widgets.noninteractive.bg_fill = Color32::WHITE;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., Color32::BLACK);
        */
        style.visuals.widgets.noninteractive.bg_fill = Color32::BLACK;
        style.visuals.widgets.noninteractive.fg_stroke =
            //Stroke::new(1., Rgba::from_rgb(1., 191. / 255., 0.)); // amber
            Stroke::new(1., Rgba::from_rgb(1., 126. / 255., 0.)); // SAE/ECE amber
        style.spacing.tooltip_width = 100.; // minimum distance of tooltip from right border (default:400)
        style.interaction.show_tooltips_only_when_still = false;
        ctx.set_style(style);
    }

    fn max_size_points(&self) -> Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        //Vec2::new(1024.0, 2048.0)
        Vec2::new(4096.0, 4096.0)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.collapsing("Layer Visibility", |ui| {
                layer_controls::show(ui, &mut self.control_state);
            });
            CollapsingHeader::new("Scenarios")
                .default_open(true)
                .show(ui, |ui| {
                    scenario_controls::show(ui, &mut self.world);
                });
            CollapsingHeader::new("Integrators")
                .default_open(true)
                .show(ui, |ui| {
                    integrator_controls::show(ui, &mut self.world);
                });
            CollapsingHeader::new("Step Sizes")
                .default_open(true)
                .show(ui, |ui| step_size_controls::show(ui, &mut self.world));
            ui.collapsing("Colors", |ui| {
                color_controls::show(ui, &mut self.control_state);
            });
            //settings::show(ui, &mut self.control_state);
        });

        CentralPanel::default().show(ctx, |ui| {
            canvas_grid::show(ui, &mut self.world, &self.control_state);
        });
    }
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: World::default(),
            //resources: Resources::default(),
            control_state: ControlState::default(),
        }
    }

    fn initialize_scenario(&mut self) {
        let step_size = Rc::clone(self.world.add_step_size(StepSize {
            user_label: UserLabel("default".to_string()),
            duration: Duration(R32::from(0.5)),
            color: Hsva::from(Color32::YELLOW),
        }));

        let _exact_for_const = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::exact_for_const::ExactForConst::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::BLUE)),
        });

        let _explicit_euler = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::euler::Broken::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::from_rgb(255, 0, 255))), // 255,0,255: magenta
        });

        let _mid_point_euler = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::Euler::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::YELLOW)),
        });

        let _mid_point_second_order = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::SecondOrder::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::GREEN)),
        });

        let implicit_euler = Rc::clone(self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::euler::Euler::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::RED)),
        }));

        let scenario_center_mass = Rc::clone(self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::CenterMass),
            start_position: StartPosition(Vec3::new(0., 1., 0.)),
            start_velocity: StartVelocity(Vec3::new(1., 0., 0.)),
            duration: Duration(std::f32::consts::TAU.into()),
        }));

        let _scenario_constant_acceleration = self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::ConstantAcceleration),
            start_position: StartPosition(Vec3::new(0., 0., 0.)),
            start_velocity: StartVelocity(Vec3::new(1., 0., 0.)),
            duration: Duration(2_f32.into()),
        });

        let canvas_center_mass = self.world.add_canvas(Canvas::new(scenario_center_mass));

        canvas_center_mass
            .borrow_mut()
            .add_integration(Integration::new(implicit_euler, step_size));
    }
}

pub struct ControlState {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub format_precision: usize,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            layerflags: LayerFlags::default(),
            strokes: Strokes::default(),
            format_precision: 3,
        }
    }
}

impl ControlState {
    #[must_use]
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

pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
    pub inspector: bool,
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

pub struct Strokes {
    pub trajectory: Stroke,
    pub acceleration: Stroke,
    pub coordinates: Stroke,
    pub focussed_velocity: Stroke,
    pub focussed_acceleration: Stroke,
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
        }
    }
}
