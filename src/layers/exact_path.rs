use std::borrow::BorrowMut;

use bevy::prelude::*;

use crate::{
    canvas::CanvasId, scenarios::ScenarioId, Canvas, IntegrationParameters, Scenario, UIState,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_exact_path.system());
    }
}

// UIState must be requested as Mut, or else it panics when other systems use it in parallel
pub fn render_exact_path(
    ui_state: ResMut<UIState>,
    integration_views: Query<(&IntegrationParameters, &ScenarioId, &CanvasId)>,
    mut scenarios: Query<&mut Scenario>,
    mut canvases: Query<&mut Canvas>,
) {
    for (params, scenario_id, canvas_id) in integration_views.iter() {
        let mut scenario = scenarios.get_mut(scenario_id.0).unwrap();
        let mut canvas = canvases.get_mut(canvas_id.0).unwrap();
        scenario.draw_on(
            canvas.borrow_mut(),
            params,
            ui_state.strokes.trajectory,
            ui_state.colors.exact_sample.into(),
        );
    }
}
