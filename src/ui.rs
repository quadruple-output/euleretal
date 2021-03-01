use crate::{canvas, integrator, step_size, Acceleration, Duration};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use core::fmt;
use egui::{stroke_ui, widgets::Slider, CentralPanel, Color32, Rgba, SidePanel, Stroke, Ui, Vec2};

pub struct Plugin;

pub struct State {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub format_precision: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            layerflags: LayerFlags::default(),
            strokes: Strokes::default(),
            format_precision: 3,
        }
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(State::default())
            .add_system(render.system());

        let ctx = &app.resources().get_mut::<EguiContext>().unwrap().ctx;
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
        ctx.set_style(style);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::borrowed_box)]
pub fn render(
    context: Res<EguiContext>,
    mut ui_state: ResMut<State>,
    mut canvases: Query<(&canvas::Kind, &mut canvas::comp::State)>,
    mut step_sizes: Query<(
        &mut step_size::Kind,
        &mut step_size::comp::UserLabel,
        &mut step_size::comp::Duration,
        &mut step_size::comp::Color,
    )>,
    mut integrators: Query<(&Box<dyn integrator::Integrator>, &mut Stroke)>,
    mut scenarios: Query<(&Box<dyn Acceleration>, &mut Duration)>,
) {
    let ctx = &context.ctx;

    /*     let mut style = (*ctx.style()).clone();
    /* -=- Change Color Scheme to B/W -=- *\
    style.visuals.widgets.noninteractive.bg_fill = Color32::WHITE;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., Color32::BLACK);
    */
    style.visuals.widgets.noninteractive.bg_fill = Color32::BLACK;
    ctx.set_style(style); */

    SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
        ui.heading("Layer Visibility");
        ui.vertical(|ui| {
            ui.checkbox(&mut ui_state.layerflags.coordinates, "Coordinates");
            ui.checkbox(
                &mut ui_state.layerflags.acceleration_field,
                "Acceleration Field",
            );
            ui.checkbox(&mut ui_state.layerflags.inspector, "Inspector");
        });
        ui.horizontal(|ui| {
            ui.label("Display Decimals");
            ui.add(Slider::usize(&mut ui_state.format_precision, 0..=12));
            ui.label(format!("{}", ui_state.format_precision));
        });

        ui.heading("Colors");
        ui.vertical(|mut ui| {
            ui_state.strokes.show_controls(&mut ui);
        });

        ui.heading("Step Sizes");
        for (_, mut label, mut duration, mut color) in step_sizes.iter_mut() {
            step_size::show_controls(ui, &mut label, &mut duration, &mut color);
        }

        ui.heading("Integrators");
        for (integrator, mut stroke) in integrators.iter_mut() {
            stroke_ui(ui, &mut stroke, &(*integrator.label()));
        }

        ui.heading("Scenarios");
        for (acceleration, mut duration) in scenarios.iter_mut() {
            let todo = "create method scenario.show_controls() (requires mut scenario)";
            ui.horizontal(|ui| {
                ui.label(acceleration.label());
                duration.show_controls(ui);
            });
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        let panel_size = ui.available_size_before_wrap_finite();
        let canvas_count = canvases.iter_mut().count();
        let canvas_size = Vec2::new(panel_size.x, panel_size.y / canvas_count as f32);
        for (_, mut canvas) in canvases.iter_mut() {
            canvas.allocate_painter(ui, canvas_size);
        }
    });
}

impl State {
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

impl Strokes {
    fn show_controls(&mut self, ui: &mut Ui) {
        stroke_ui(ui, &mut self.trajectory, "Exact Trajectory");
        stroke_ui(ui, &mut self.acceleration, "Acceleration (Field)");
        stroke_ui(ui, &mut self.coordinates, "Coordinates");
        stroke_ui(ui, &mut self.focussed_acceleration, "Acceleration");
        stroke_ui(ui, &mut self.focussed_velocity, "Velocity");
    }
}
