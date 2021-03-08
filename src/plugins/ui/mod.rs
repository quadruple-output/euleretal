use crate::prelude::*;
use ::bevy::prelude::*;
use ::bevy_egui::EguiContext;
use ::core::fmt;
use ::egui::{CentralPanel, Rgba, SidePanel};

mod canvas_grid;
mod canvas_view;
mod color_controls;
mod integrator_controls;
mod layer_controls;
mod scenario_controls;
mod settings;
mod step_size_controls;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(State::default())
            .add_startup_system(setup.system())
            .add_system(show.system());
    }
}

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

#[allow(clippy::needless_pass_by_value)]
pub fn setup(context: Res<EguiContext>) {
    let ctx = &context.ctx;
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

#[allow(clippy::needless_pass_by_value, clippy::borrowed_box)]
pub fn show(
    context: Res<EguiContext>,
    mut state: ResMut<State>,
    mut canvases: Query<&mut canvas::comp::State>,
    mut step_sizes: Query<(
        &mut step_size::comp::UserLabel,
        &mut step_size::comp::Duration,
        &mut step_size::comp::Color,
    )>,
    mut integrators: Query<(&Box<dyn Integrator>, &mut Stroke)>,
    mut scenarios: Query<(&Box<dyn Acceleration>, &mut Duration)>,
) {
    let ctx = &context.ctx;

    SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
        layer_controls::show(ui, &mut state);
        settings::show(ui, &mut state);
        color_controls::show(ui, &mut state);
        step_size_controls::show(ui, &mut step_sizes);
        integrator_controls::show(ui, &mut integrators);
        scenario_controls::show(ui, &mut scenarios);
    });

    CentralPanel::default().show(ctx, |ui| {
        canvas_grid::show(ui, &mut canvases);
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
