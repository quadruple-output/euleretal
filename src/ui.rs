use crate::canvas::Canvas;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Sense, SidePanel};

pub struct Plugin;

#[derive(Default)]
pub struct UIState {
    pub layerflags: LayerFlags,
    pub canvas: Option<Canvas>,
}

pub struct LayerFlags {
    pub coordinates: bool,
    pub acceleration_field: bool,
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UIState::default())
            .add_system(render.system());
    }
}

pub fn render(context: Res<EguiContext>, mut state: ResMut<UIState>) {
    let ctx = &context.ctx;
    SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
        ui.heading("Layer Visibility");
        ui.vertical(|ui| {
            ui.checkbox(&mut state.layerflags.coordinates, "Coordinates");
            ui.checkbox(&mut state.layerflags.acceleration_field, "Accelerometer");
        })
    });

    CentralPanel::default().show(ctx, |ui| {
        state.canvas = Some(Canvas::new(
            ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::click()),
            8.,
        ));
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
