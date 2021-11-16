use super::{
    core::Scenario,
    entities::{Canvas, Integrator, StepSize},
    misc::{entity_store, Settings},
};
use ::std::{cell::RefCell, slice::Iter};

#[derive(Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct World {
    canvases: Vec<RefCell<Canvas>>,
    scenarios: entity_store::List<Scenario>,
    integrators: entity_store::List<Integrator>,
    step_sizes: entity_store::List<StepSize>,
    pub settings: Settings,
}

impl World {
    pub fn canvases(&self) -> Iter<RefCell<Canvas>> {
        self.canvases.iter()
    }

    pub fn scenarios(&self) -> &entity_store::List<Scenario> {
        &self.scenarios
    }

    pub fn integrators(&self) -> &entity_store::List<Integrator> {
        &self.integrators
    }

    pub fn step_sizes(&self) -> &entity_store::List<StepSize> {
        &self.step_sizes
    }

    pub fn add_canvas(&mut self, canvas: Canvas) -> &RefCell<Canvas> {
        self.canvases.push(RefCell::new(canvas));
        self.canvases.last().unwrap()
    }

    pub fn add_scenario(&mut self, scenario: Scenario) -> entity_store::Index<Scenario> {
        self.scenarios.push(scenario)
    }

    pub fn add_step_size(&mut self, step_size: StepSize) -> entity_store::Index<StepSize> {
        self.step_sizes.push(step_size)
    }

    pub fn remove_step_size(&mut self, step_size_idx: entity_store::Index<StepSize>) {
        self.step_sizes.delete(step_size_idx);
    }

    pub fn add_integrator(&mut self, integrator: Integrator) -> entity_store::Index<Integrator> {
        self.integrators.push(integrator)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_canvas(&mut self, canvas: *const RefCell<Canvas>) {
        self.canvases
            .retain(|candidate| !::std::ptr::eq(canvas, candidate));
    }
}

impl ::std::ops::Index<entity_store::Index<Integrator>> for World {
    type Output = RefCell<Integrator>;

    fn index(&self, index: entity_store::Index<Integrator>) -> &Self::Output {
        &self.integrators[index]
    }
}

impl ::std::ops::Index<entity_store::Index<Scenario>> for World {
    type Output = RefCell<Scenario>;

    fn index(&self, index: entity_store::Index<Scenario>) -> &Self::Output {
        &self.scenarios[index]
    }
}

impl ::std::ops::Index<entity_store::Index<StepSize>> for World {
    type Output = RefCell<StepSize>;

    fn index(&self, index: entity_store::Index<StepSize>) -> &Self::Output {
        &self.step_sizes[index]
    }
}
