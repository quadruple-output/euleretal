use bevy::ecs::Bundle;

use crate::{CanvasId, IntegrationParameters, IntegratorId, ScenarioId};

#[derive(Bundle)]
pub struct IntegrationViewBundle {
    pub scenario_id: ScenarioId,
    pub integrator_id: IntegratorId,
    pub parameters: IntegrationParameters,
    pub ui_state: IntegrationViewState,
    pub canvas_id: CanvasId,
}

#[derive(Default)]
pub struct IntegrationViewState {
    _dummy: u8,
}
