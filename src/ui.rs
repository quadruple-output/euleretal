use crate::{canvas::Canvas, scenario::Scenario};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Sense, SidePanel};

pub struct Plugin;

#[derive(Default)]
pub struct UIState {
    scenario: Scenario,
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
            ui.heading("Controls");
            ui.vertical(|ui| {
                ui.label("Scenario");
                ui.radio_value(
                    &mut state.scenario,
                    Scenario::LinearAccel,
                    "Uniform acceleration",
                );
                ui.radio_value(&mut state.scenario, Scenario::Rotation, "Rotation");
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
