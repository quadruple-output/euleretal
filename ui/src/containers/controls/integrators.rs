use super::{misc::my_stroke_ui, ui_import::Ui, World};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    world.integrators().iter().for_each(|integrator| {
        let (label, description);
        {
            let core_integrator = &integrator.borrow().core;
            label = core_integrator.label();
            description = core_integrator.description();
        }
        my_stroke_ui(
            ui,
            &mut integrator.borrow_mut().stroke,
            &label,
            &description,
        );
    });
}
