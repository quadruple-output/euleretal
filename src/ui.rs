use bevy::prelude::*;
use egui::{Pos2, Rgba, Sense, Stroke, Vec2};

pub struct Plugin;

#[derive(Default)]
pub struct UIState {
    scenario: Scenario,
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(UIState::default()).add_system(ui.system());
    }
}

pub fn ui(mut state: ResMut<UIState>, mut context: ResMut<bevy_egui::EguiContext>) {
    let ctx = &mut context.ctx;
    egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
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
    egui::CentralPanel::default().show(ctx, |ui| {
        let (response, canvas) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::click());
        let area = response.rect;
        let o = area.center();
        let o = Pos2::new(o.x.round(), o.y.round());
        let x_max = area.width() * 0.5;
        let y_max = area.height() * 0.5;
        let coord_stroke = Stroke::new(1., Rgba::from_rgb(0., 0.5, 0.) * 0.7);

        canvas.line_segment([o - Vec2::X * x_max, o + Vec2::X * x_max], coord_stroke);
        canvas.line_segment([o - Vec2::Y * y_max, o + Vec2::Y * y_max], coord_stroke);
        response.on_hover_ui(|ui| {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                ui.label(format!("X:{}", (mouse_pos.x - o.x).round()));
                ui.label(format!("Y:{}", (mouse_pos.y - o.y).round()));
            }
        });
    });
}

#[derive(PartialEq)]
enum Scenario {
    LinearAccel,
    Rotation,
}

impl Default for Scenario {
    fn default() -> Self {
        Self::Rotation
    }
}
