use super::{
    core::{Obj, Scenario},
    entities::{Canvas, Integrator, StepSize},
    misc::{entity_store, Settings},
};
use ::std::{cell::RefCell, rc::Rc, slice::Iter};

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Serialize, ::serde::Deserialize)
)]
pub struct World {
    canvases: Vec<RefCell<Canvas>>,
    scenarios: entity_store::List<Scenario>,
    integrators: Vec<Obj<Integrator>>,
    step_sizes: Vec<Obj<StepSize>>,
    pub settings: Settings,
}

impl World {
    pub fn canvases(&self) -> Iter<RefCell<Canvas>> {
        self.canvases.iter()
    }

    pub fn scenarios(&self) -> &entity_store::List<Scenario> {
        &self.scenarios
    }

    pub fn integrators(&self) -> Iter<Obj<Integrator>> {
        self.integrators.iter()
    }

    pub fn step_sizes(&self) -> Iter<Obj<StepSize>> {
        self.step_sizes.iter()
    }

    pub fn add_canvas(&mut self, canvas: Canvas) -> &RefCell<Canvas> {
        self.canvases.push(RefCell::new(canvas));
        self.canvases.last().unwrap()
    }

    pub fn add_scenario(&mut self, scenario: Scenario) -> entity_store::Index<Scenario> {
        self.scenarios.push(scenario)
    }

    pub fn add_step_size(&mut self, step_size: StepSize) -> &Obj<StepSize> {
        self.step_sizes.push(Rc::new(RefCell::new(step_size)));
        self.step_sizes.last().unwrap()
    }

    pub fn add_integrator(&mut self, configured_integrator: Integrator) -> &Obj<Integrator> {
        self.integrators
            .push(Rc::new(RefCell::new(configured_integrator)));
        self.integrators.last().unwrap()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_canvas(&mut self, canvas: *const RefCell<Canvas>) {
        self.canvases
            .retain(|candidate| !::std::ptr::eq(canvas, candidate));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_sizes
            .retain(|candidate| !Rc::ptr_eq(&step_size, candidate));
    }
}
