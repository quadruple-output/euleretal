use super::{
    ui_import::{egui, egui::Slider, Ui},
    World,
};

#[allow(clippy::borrowed_box)]
pub fn show(ui: &mut Ui, world: &mut World) {
    egui::Grid::new("integrator grid")
        .striped(false)
        .show(ui, |ui| {
            // table header:
            ui.label("Duration");
            ui.label("Scenario");
            ui.end_row();

            world.scenarios().for_each(|scenario| {
                //for (acceleration, mut duration) in
                //world.query_mut::<(&Box<dyn AccelerationField>, &mut Duration)>()
                //{
                let mut duration_for_edit = scenario.borrow().duration.into();
                ui.add(Slider::new(&mut duration_for_edit, 0.1..=50.).logarithmic(true));
                scenario.borrow_mut().duration = duration_for_edit.into();

                ui.label(scenario.borrow().acceleration.label());
                ui.end_row();
            });
        });
}
