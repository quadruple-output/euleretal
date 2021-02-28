pub struct Kind;
pub mod comp {
    pub type UserLabel = crate::UserLabel;
    pub type Duration = crate::Duration;
    pub type Color = egui::color::Hsva;
}

#[derive(Clone, Copy)]
pub struct Entity(pub bevy::prelude::Entity);

#[derive(bevy::ecs::Bundle)]
pub struct Bundle(
    pub Kind,
    pub comp::UserLabel,
    pub comp::Duration,
    pub comp::Color,
);

impl Bundle {
    pub fn spawn(self, commands: &mut bevy::ecs::Commands) -> self::Entity {
        Entity(commands.spawn(self).current_entity().unwrap())
    }
}

pub fn show_controls(
    ui: &mut egui::Ui,
    label: &mut comp::UserLabel,
    duration: &mut comp::Duration,
    color: &mut comp::Color,
) {
    use decorum::R32;
    use egui::{
        color_picker::{color_edit_button_hsva, Alpha},
        Slider,
    };

    ui.horizontal(|mut ui| {
        // edit color:
        color_edit_button_hsva(&mut ui, color, Alpha::BlendOrAdditive);
        // edit label:
        ui.add(egui::TextEdit::singleline(&mut label.0).desired_width(0.));
        if label.0.is_empty() {
            label.0 = "<unnamed>".to_string();
        }
        // edit dt:
        let mut dt = duration.0.get().into_inner();
        ui.add(Slider::f32(&mut dt, 0.01..=2.).text("dt").logarithmic(true));
        duration.0.set(R32::from(dt).max(R32::from(0.01)));
    });
}
