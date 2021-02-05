use core::fmt;

use crate::canvas::Canvas;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{
    color::Hsva,
    color_picker::{color_edit_button_hsva, Alpha},
    stroke_ui,
    widgets::Slider,
    CentralPanel, Color32, Rgba, SidePanel, Stroke, Ui,
};

pub struct Plugin;

pub struct UIState {
    pub layerflags: LayerFlags,
    pub strokes: Strokes,
    pub colors: Colors,
    pub format_precision: usize,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            layerflags: Default::default(),
            strokes: Default::default(),
            colors: Default::default(),
            format_precision: 3,
        }
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UIState::default())
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
    mut ui_state: ResMut<UIState>,
    mut canvases: Query<&mut Canvas>,
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
            ui_state.colors.show_controls(&mut ui);
            ui_state.strokes.show_controls(&mut ui);
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        for mut canvas in canvases.iter_mut() {
            canvas.allocate_painter(ui, ui.available_size_before_wrap_finite());
        }
    });
}

impl UIState {
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
            acceleration_field: true,
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
pub struct Colors {
    pub exact_sample: Hsva,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            exact_sample: Hsva::from(Color32::YELLOW),
        }
    }
}

impl Colors {
    fn show_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|mut ui| {
            color_edit_button_hsva(&mut ui, &mut self.exact_sample, Alpha::BlendOrAdditive);
            ui.label("Exact Sample Points");
        });
    }
}
