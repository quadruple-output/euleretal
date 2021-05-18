use crate::prelude::*;
use std::{cell::RefCell, rc::Rc, slice::Iter};

#[derive(Default)]
pub struct World {
    canvases: Vec<Obj<Canvas>>,
    scenarios: Vec<Obj<Scenario>>,
    configured_integrators: Vec<Obj<ui::Integrator>>,
    step_sizes: Vec<Obj<StepSize>>,
    // integrations are not here! They are managed by their canvases.
}

impl World {
    pub fn canvases(&self) -> Iter<Obj<Canvas>> {
        self.canvases.iter()
    }

    pub fn scenarios(&self) -> Iter<Obj<Scenario>> {
        self.scenarios.iter()
    }

    pub fn configured_integrators(&self) -> Iter<Obj<ui::Integrator>> {
        self.configured_integrators.iter()
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

    pub fn add_configured_integrator(
        &mut self,
        configured_integrator: ui::Integrator,
    ) -> &Obj<ui::Integrator> {
        self.configured_integrators
            .push(Rc::new(RefCell::new(configured_integrator)));
        self.configured_integrators.last().unwrap()
    }

    pub fn remove_canvas(&mut self, canvas: Obj<Canvas>) {
        self.canvases
            .retain(|candidate| !Rc::ptr_eq(&canvas, candidate));
    }

    pub fn remove_step_size(&mut self, step_size: Obj<StepSize>) {
        self.step_sizes
            .retain(|candidate| !Rc::ptr_eq(&step_size, candidate));
    }
}
