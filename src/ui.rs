use core::fmt;

use crate::{Canvas, ConfiguredIntegrator, Scenario, StepSize};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{stroke_ui, widgets::Slider, CentralPanel, Color32, Rgba, SidePanel, Stroke, Ui};

pub struct Plugin;

pub struct UiState {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub format_precision: usize,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            layerflags: Default::default(),
            strokes: Default::default(),
            format_precision: 3,
        }
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UiState::default())
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

pub fn render(
    context: Res<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut canvases: Query<&mut Canvas>,
    mut step_sizes: Query<&mut StepSize>,
    mut integrators: Query<&mut ConfiguredIntegrator>,
    mut scenarios: Query<&mut Scenario>,
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
        for mut step_size in step_sizes.iter_mut() {
            step_size.show_controls(ui);
        }

        ui.heading("Integrators");
        for mut integrator in integrators.iter_mut() {
            integrator.show_controls(ui);
        }

        ui.heading("Scenarios");
        for mut scenario in scenarios.iter_mut() {
            scenario.show_controls(ui);
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        for mut canvas in canvases.iter_mut() {
            canvas.allocate_painter(ui, ui.available_size_before_wrap_finite());
        }
    });
}

impl UiState {
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
