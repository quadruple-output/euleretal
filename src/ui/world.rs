use ::std::{cell::RefCell, rc::Rc, slice::Iter};

use super::{
    core::{Obj, Scenario},
    entities::{Canvas, Integrator, StepSize},
    misc::Settings,
};

#[derive(Default)]
pub struct World {
    canvases: Vec<Obj<Canvas>>,
    scenarios: Vec<Obj<Scenario>>,
    integrators: Vec<Obj<Integrator>>,
    step_sizes: Vec<Obj<StepSize>>,
    pub settings: Settings,
}

impl World {
    pub fn canvases(&self) -> Iter<Obj<Canvas>> {
        self.canvases.iter()
    }

    pub fn scenarios(&self) -> Iter<Obj<Scenario>> {
        self.scenarios.iter()
    }

    pub fn integrators(&self) -> Iter<Obj<Integrator>> {
        self.integrators.iter()
    }

    pub fn step_sizes(&self) -> Iter<Obj<StepSize>> {
        self.step_sizes.iter()
    }

    pub fn add_canvas(&mut self, canvas: Canvas) -> &Obj<Canvas> {
        self.canvases.push(Rc::new(RefCell::new(canvas)));
        self.canvases.last().unwrap()
    }

    pub fn add_scenario(&mut self, scenario: Scenario) -> &Obj<Scenario> {
        self.scenarios.push(Rc::new(RefCell::new(scenario)));
        self.scenarios.last().unwrap()
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
    pub fn remove_canvas(&mut self, canvas: Obj<Canvas>) {
        self.canvases
            .retain(|candidate| !Rc::ptr_eq(&canvas, candidate));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_sizes
            .retain(|candidate| !Rc::ptr_eq(&step_size, candidate));
    }
}
