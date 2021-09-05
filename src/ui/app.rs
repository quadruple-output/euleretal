use ::std::rc::Rc;

use super::{
    containers,
    core::{Scenario, StartPosition, StartVelocity},
    entities::{Canvas, Integration, Integrator, StepSize},
    import::Vec3,
    integrators,
    misc::UserLabel,
    scenarios,
    ui_import::{
        egui::{self, color::Hsva, CentralPanel, Color32, Rgba, SidePanel, Stroke, Vec2},
        epi,
    },
    World,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Euleretal {
    world: World,
}

impl Default for Euleretal {
    fn default() -> Self {
        let mut default = Self::new();
        default.initialize_scenario();
        default
    }
}

impl epi::App for Euleretal {
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

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
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
        SidePanel::left("side_panel").show(ctx, |ui| {
            containers::controls::show(ui, &mut self.world);
            containers::settings::show(ui, &mut self.world.settings);
        });

        CentralPanel::default().show(ctx, |ui| {
            containers::canvas::grid::show(ui, &mut self.world);
        });
    }
}

impl Euleretal {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: World::default(),
        }
    }

    fn initialize_scenario(&mut self) {
        let step_size = Rc::clone(self.world.add_step_size(StepSize {
            user_label: UserLabel("default".to_string()),
            duration: 0.11.into(),
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

        let mid_point_euler = Rc::clone(self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::Euler::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::YELLOW)),
        }));

        let _mid_point_second_order = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::SecondOrder::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::GREEN)),
        });

        let _implicit_euler = Rc::clone(self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::euler::Euler::new()),
            stroke: Stroke::new(1., Hsva::from(Color32::RED)),
        }));

        let scenario_center_mass = Rc::clone(self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::CenterMass),
            start_position: StartPosition(Vec3::new(0., 1., 0.)),
            start_velocity: StartVelocity(Vec3::new(1., 0., 0.)),
            duration: std::f32::consts::TAU.into(),
        }));

        let _scenario_constant_acceleration = self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::ConstantAcceleration),
            start_position: StartPosition(Vec3::new(0., 0., 0.)),
            start_velocity: StartVelocity(Vec3::new(1., 0., 0.)),
            duration: 2_f32.into(),
        });

        let canvas_center_mass = self.world.add_canvas(Canvas::new(scenario_center_mass));

        canvas_center_mass
            .borrow_mut()
            .add_integration(Integration::new(mid_point_euler, step_size));
    }
}
