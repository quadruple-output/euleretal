use crate::prelude::*;

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    for (integrator, mut stroke) in world.query_mut::<(&Box<dyn Integrator>, &mut Stroke)>() {
        my_stroke_ui(
            ui,
            &mut stroke,
            &integrator.label(),
            &integrator.description(),
        );
    }
}
