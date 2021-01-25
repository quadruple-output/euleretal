use crate::canvas::Canvas;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Sense, SidePanel};

pub struct Plugin;

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

#[derive(Default)]
pub struct UIState {
    pub layers: LayerFlags,
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UIState::default())
            .add_system(system.system());
    }
}

pub fn system(_world: &mut World, resources: &mut Resources) {
    let mut canvas: Option<Canvas> = None;
    {
        let ctx = &resources.get::<EguiContext>().unwrap().ctx;
        let mut state = resources.get_mut::<UIState>().unwrap();
        SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Layers");
            ui.vertical(|ui| {
                ui.checkbox(&mut state.layers.coordinates, "Coordinates");
                ui.checkbox(&mut state.layers.acceleration_field, "Accelerometer");
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            canvas = Some(Canvas::new(
                ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::click()),
                8.,
            ));
        });
    }
    resources.insert(canvas.unwrap());
}
