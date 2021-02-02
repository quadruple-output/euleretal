use crate::canvas::Canvas;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Color32, Rgba, SidePanel, Stroke};

pub struct Plugin;

#[derive(Default)]
pub struct UIState {
    pub layerflags: LayerFlags,
    pub canvas: Canvas,
    pub strokes: Strokes,
    pub colors: Colors,
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

pub fn render(context: Res<EguiContext>, mut state: ResMut<UIState>) {
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
            ui.checkbox(&mut state.layerflags.coordinates, "Coordinates");
            ui.checkbox(&mut state.layerflags.acceleration_field, "Accelerometer");
        });
        ui.heading("Colors");
    });

    CentralPanel::default().show(ctx, |ui| {
        state
            .canvas
            .allocate_painter(ui, ui.available_size_before_wrap_finite());
    });
}

pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
}

impl Default for LayerFlags {
    fn default() -> Self {
        Self {
            coordinates: true,
            acceleration_field: true,
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

pub struct Colors {
    pub exact_sample: Color32,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            exact_sample: Color32::YELLOW,
        }
    }
}
