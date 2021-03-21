use crate::prelude::*;
use bevy_ecs::World;

pub fn render(world: &mut World, state: &ControlState) {
    if !state.layerflags.acceleration_field {
        return;
    }

    for (canvas, scenario_id) in world.query::<(&canvas::State, &canvas::comp::ScenarioId)>() {
        let acceleration = world
            .get::<scenario::comp::Acceleration>(scenario_id.0)
            .unwrap();

        let min = canvas.min();
        let max = canvas.max();
        for x in ((min.x - 1.) as i32)..=((max.x + 1.) as i32) {
            for y in ((min.y - 1.) as i32)..=((max.y + 1.) as i32) {
                let pos = Vec3::new(x as f32, y as f32, 0.);
                let a = acceleration.value_at(pos);
                canvas.draw_vector(pos, a, state.strokes.acceleration)
            }
        }

        canvas.on_hover_ui(|ui, mouse_pos| {
            let a = acceleration.value_at(mouse_pos);
            ui.label("Field");
            ui.separator();
            ui.label(format!("|a| = {}", state.format_f32(a.length())));
            canvas.draw_vector(mouse_pos, a, state.strokes.acceleration)
        })
    }
}
