use super::core::{Duration, Position, Scenario};
use ::std::{collections::hash_map::DefaultHasher, hash::Hasher};

#[derive(Default)]
pub struct TrajectoryBuffer {
    trajectory: Vec<Position>,
    scenario_hash: u64,
    trajectory_min_dt: Duration,
}

impl ::std::ops::Deref for TrajectoryBuffer {
    type Target = Vec<Position>;

    fn deref(&self) -> &Self::Target {
        &self.trajectory
    }
}

impl TrajectoryBuffer {
    pub fn new(scenario: &Scenario, min_dt: Duration) -> Self {
        Self {
            trajectory: scenario.calculate_trajectory(min_dt),
            trajectory_min_dt: min_dt,
            scenario_hash: Self::hash_scenario(scenario),
        }
    }

    pub fn hash_scenario(scenario: &Scenario) -> u64 {
        let mut hasher = DefaultHasher::new();
        scenario.hash_default(&mut hasher);
        hasher.finish()
    }

    pub fn update_trajectory(&mut self, scenario: &Scenario, min_dt: Duration) {
        let scenario_hash = Self::hash_scenario(scenario);
        if self.scenario_hash != scenario_hash || self.trajectory_min_dt > min_dt {
            self.trajectory = scenario.calculate_trajectory(min_dt);
            self.trajectory_min_dt = min_dt;
            self.scenario_hash = scenario_hash;
        }
    }
}
