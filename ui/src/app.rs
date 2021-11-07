use super::{
    containers,
    core::{Position, Scenario, Velocity},
    entities::{Canvas, Integration, Integrator, StepSize},
    integrators,
    misc::UserLabel,
    scenarios,
    ui_import::{
        egui::{self, CentralPanel, SidePanel},
        epi, Color32, Hsva, Rgba, Stroke, Vec2,
    },
    World,
};
use ::std::{rc::Rc, time::Instant};

pub struct Euleretal {
    world: World,
    last_update: Option<Instant>,
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

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        println!("saving app state");
        epi::set_value(storage, epi::APP_KEY, &self.world);
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[allow(clippy::used_underscore_binding)]
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            println!("loading app state");
            let option = storage.get_string(epi::APP_KEY);
            if let Some(string) = option {
                let result = ::ron::from_str::<World>(&string);
                dbg!(result);
            }
            if let Some(saved_world) = epi::get_value(storage, epi::APP_KEY) {
                self.world = saved_world;
                return;
            }
            println!("…not found");
        }
        Self::init_display_style(ctx);
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
        if let Some(last_update) = self.last_update {
            let micros = last_update.elapsed().as_micros();
            if micros > 50000 {
                log::debug!("Frame: {}µs", last_update.elapsed().as_micros());
            }
        }
        self.last_update = Some(Instant::now());
        if ctx.input().key_pressed(egui::Key::Q) {
            _frame.quit();
        }
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {
        println!("exiting");
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

impl Euleretal {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: World::default(),
            last_update: None,
        }
    }

    fn initialize_scenario(&mut self) {
        let step_size = Rc::clone(self.world.add_step_size(StepSize {
            user_label: UserLabel("default".to_string()),
            duration: 0.11.into(),
            color: Color32::YELLOW,
        }));

        let _exact_for_const = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::exact_for_const::ExactForConst),
            stroke: Stroke::new(1., Hsva::from(Color32::BLUE)),
        });

        let _explicit_euler = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::euler::Broken),
            stroke: Stroke::new(1., Hsva::from(Color32::from_rgb(255, 0, 255))), // 255,0,255: magenta
        });

        let mid_point_euler = Rc::clone(self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::Euler),
            stroke: Stroke::new(1., Hsva::from(Color32::YELLOW)),
        }));

        let _mid_point_second_order = self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::mid_point::SecondOrder),
            stroke: Stroke::new(1., Hsva::from(Color32::GREEN)),
        });

        let _implicit_euler = Rc::clone(self.world.add_integrator(Integrator {
            integrator: Box::new(integrators::euler::Euler),
            stroke: Stroke::new(1., Hsva::from(Color32::RED)),
        }));

        let scenario_center_mass = Rc::clone(self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::CenterMass),
            start_position: Position::new(0., 1., 0.),
            start_velocity: Velocity::new(1., 0., 0.),
            duration: std::f32::consts::TAU.into(),
        }));

        let _scenario_constant_acceleration = self.world.add_scenario(Scenario {
            acceleration: Box::new(scenarios::ConstantAcceleration),
            start_position: Position::origin(),
            start_velocity: Velocity::new(1., 0., 0.),
            duration: 2_f32.into(),
        });

        let canvas_center_mass = self.world.add_canvas(Canvas::new(scenario_center_mass));

        canvas_center_mass
            .borrow_mut()
            .add_integration(Integration::new(mid_point_euler, step_size));
    }

    fn init_display_style(ctx: &egui::CtxRef) {
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
}
