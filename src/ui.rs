use crate::canvas::Canvas;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Color32, Rgba, SidePanel, Stroke};

pub struct Plugin;

#[derive(Default)]
pub struct UIState {
    pub layerflags: LayerFlags,
    pub canvas: Canvas,
}

pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
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
        })
    });

    CentralPanel::default().show(ctx, |ui| {
        state
            .canvas
            .allocate_painter(ui, ui.available_size_before_wrap_finite());
    });
}

impl Default for LayerFlags {
    fn default() -> Self {
        Self {
            coordinates: true,
            acceleration_field: true,
        }
    }
}
