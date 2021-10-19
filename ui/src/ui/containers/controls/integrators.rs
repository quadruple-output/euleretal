use super::{misc::my_stroke_ui, ui_import::Ui, World};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    world.integrators().for_each(|integrator| {
        let label = integrator.borrow().integrator.label();
        let description = integrator.borrow().integrator.description();
        my_stroke_ui(
            ui,
            &mut integrator.borrow_mut().stroke,
            &label,
            &description,
        );
    });
}
