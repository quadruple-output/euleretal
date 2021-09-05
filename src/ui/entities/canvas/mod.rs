mod painter;

pub use self::painter::Painter;
use super::{
    core::{Duration, Obj, Position, Samples, Scenario},
    import::Vec3,
    misc::{BoundingBox, PointFormat},
    ui_import::{egui, Color32, Pos2, Ui, Vec2},
    Integration,
};
use ::std::{
    cell::RefCell, collections::hash_map::DefaultHasher, hash::Hasher, rc::Rc, slice::Iter,
};

pub struct Canvas {
    scenario: Obj<Scenario>,
    integrations: Vec<Obj<Integration>>,
    visible_units: f32,
    focus: Position,
    scale: Vec3,
    area_center: Pos2,
    trajectory_buffer: Option<TrajectoryBuffer>,
    pub ui_integrations_window_is_open: bool,
}

pub trait ObjExtras {
    fn allocate_painter(&self, ui: &mut Ui, size: Vec2) -> Painter;
}

impl ObjExtras for Obj<Canvas> {
    fn allocate_painter(&self, ui: &mut Ui, size: Vec2) -> Painter {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        Painter::new(self, response, painter)
    }
}

impl Canvas {
    pub fn new(scenario: Obj<Scenario>) -> Self {
        Self {
            scenario,
            integrations: Vec::default(),
            visible_units: 1.,
            focus: Vec3::default(),
            scale: Vec3::default(),
            area_center: Pos2::default(),
            trajectory_buffer: None,
            ui_integrations_window_is_open: false,
        }
    }

    pub fn scenario(&self) -> &Obj<Scenario> {
        &self.scenario
    }

    pub fn set_scenario(&mut self, new_scenario: Obj<Scenario>) {
        self.scenario = new_scenario;
        self.trajectory_buffer = None;
    }

    pub fn integrations(&self) -> Iter<Obj<Integration>> {
        self.integrations.iter()
    }

    pub fn add_integration(&mut self, integration: Integration) {
        self.integrations.push(Rc::new(RefCell::new(integration)));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_integration(&mut self, integration: Obj<Integration>) {
        self.integrations
            .retain(|candidate| !Rc::ptr_eq(candidate, &integration));
    }

    pub fn update_trajectory(&mut self, min_dt: Duration) {
        if let Some(ref mut buffer) = self.trajectory_buffer {
            buffer.update_trajectory(&self.scenario.borrow(), min_dt);
        } else {
            self.trajectory_buffer = Some(TrajectoryBuffer::new(&self.scenario.borrow(), min_dt));
        }
    }

    #[must_use]
    pub fn has_trajectory(&self) -> bool {
        self.trajectory_buffer.is_some()
    }

    #[must_use]
    pub fn bbox(&self) -> Option<BoundingBox> {
        self.trajectory_buffer.as_ref().map(|buf| {
            let mut bbox = BoundingBox::default();
            buf.trajectory.iter().for_each(|&s| bbox.expand_to(s));
            bbox
        })
    }

    pub fn set_visible_bbox(&mut self, bbox: &BoundingBox) {
        self.focus = bbox.center();
        self.visible_units = bbox.diameter() * 1.2;
    }

    fn adjust_scale_and_center(&mut self, paint_area: &egui::Rect) {
        let scale = f32::min(paint_area.width(), paint_area.height()) / self.visible_units;
        self.scale = Vec3::new(scale, -scale, 1.);
        self.area_center = paint_area.center();
    }

    fn user_to_screen(&self, pos: Position) -> Pos2 {
        ((pos - self.focus).component_mul(&self.scale)).to_pos2() + self.area_center.to_vec2()
    }

    fn screen_to_user(&self, pos: Pos2) -> Position {
        debug_assert!(self.scale != Vec3::default());
        (pos - self.area_center.to_vec2())
            .to_vec3()
            .component_div(&self.scale)
            + self.focus
    }
}

#[derive(Default)]
struct TrajectoryBuffer {
    trajectory: Vec<Vec3>,
    scenario_hash: u64,
    trajectory_min_dt: Duration,
}

impl TrajectoryBuffer {
    fn new(scenario: &Scenario, min_dt: Duration) -> Self {
        Self {
            trajectory: scenario.calculate_trajectory(min_dt),
            trajectory_min_dt: min_dt,
            scenario_hash: Self::hash_scenario(scenario),
        }
    }

    fn hash_scenario(scenario: &Scenario) -> u64 {
        let mut hasher = DefaultHasher::new();
        scenario.hash_default(&mut hasher);
        hasher.finish()
    }

    fn update_trajectory(&mut self, scenario: &Scenario, min_dt: Duration) {
        let scenario_hash = Self::hash_scenario(scenario);
        if self.scenario_hash != scenario_hash || self.trajectory_min_dt > min_dt {
            self.trajectory = scenario.calculate_trajectory(min_dt);
            self.trajectory_min_dt = min_dt;
            self.scenario_hash = scenario_hash;
        }
    }
}

trait ToPos2 {
    fn to_pos2(&self) -> Pos2;
}

trait ToVec3 {
    fn to_vec3(&self) -> Vec3;
}

impl ToVec3 for Pos2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

impl ToVec3 for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

impl ToPos2 for Vec3 {
    fn to_pos2(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }
}
