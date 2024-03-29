use super::{
    core::{Duration, Scenario},
    import::{Point3, Vec3},
    misc::{entity_store, BoundingBox},
    trajectory_buffer::TrajectoryBuffer,
    ui_import::{egui, Pos2, Ui, Vec2},
    Integration, Painter, World,
};
use ::std::cell::RefCell;

#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct Canvas {
    scenario: entity_store::Index<Scenario>,
    scenario_is_new: bool,
    pub(super) integrations: Vec<RefCell<Integration>>, // todo: should not be public (or explained)
    visible_units: f32,
    focus: Point3,
    scale: Vec3,
    area_center: Pos2,
    pub ui_integrations_window_is_open: bool,
    #[serde(skip)]
    pub(super) trajectory_buffer: Option<TrajectoryBuffer>, // todo: should not be public (or explained)
}

impl ::std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas")
            .field("scenario", &self.scenario)
            .field("integrations", &self.integrations)
            .field("visible_units", &self.visible_units)
            .field("focus", &self.focus)
            .field("scale", &self.scale)
            .field("area_center", &self.area_center)
            //.field("trajectory_buffer", &self.trajectory_buffer)
            .field(
                "ui_integrations_window_is_open",
                &self.ui_integrations_window_is_open,
            )
            .finish()
    }
}

pub trait ObjExtras {
    fn allocate_painter(&self, ui: &mut Ui, size: Vec2) -> Painter;
}

impl ObjExtras for RefCell<Canvas> {
    fn allocate_painter(&self, ui: &mut Ui, size: Vec2) -> Painter {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        Painter::new(self, response, painter)
    }
}

impl Canvas {
    pub fn new(scenario_idx: entity_store::Index<Scenario>) -> Self {
        Self {
            scenario: scenario_idx,
            scenario_is_new: true,
            integrations: Vec::default(),
            visible_units: 1.,
            focus: Point3::origin(),
            scale: Vec3::default(),
            area_center: Pos2::default(),
            trajectory_buffer: None,
            ui_integrations_window_is_open: false,
        }
    }

    pub fn scenario_idx(&self) -> entity_store::Index<Scenario> {
        self.scenario
    }

    pub fn set_scenario(&mut self, new_scenario: entity_store::Index<Scenario>) {
        self.scenario = new_scenario;
        self.scenario_is_new = true;
        self.trajectory_buffer = None;
    }

    pub fn integrations(&self) -> ::core::slice::Iter<RefCell<Integration>> {
        self.integrations.iter()
    }

    pub fn add_integration(&mut self, integration: Integration) {
        self.integrations.push(RefCell::new(integration));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_integration(&mut self, integration_idx: usize) {
        self.integrations.remove(integration_idx);
    }

    pub fn integration_at(&self, integration_idx: usize) -> &RefCell<Integration> {
        &self.integrations[integration_idx]
    }

    pub fn update_trajectory(&mut self, scenario: &Scenario, min_dt: Duration) {
        if let Some(ref mut buffer) = self.trajectory_buffer {
            buffer.update_trajectory(scenario, min_dt);
        } else {
            self.trajectory_buffer = Some(TrajectoryBuffer::new(scenario, min_dt));
        }
    }

    pub fn scenario_is_new_once(&mut self) -> bool {
        let result = self.scenario_is_new;
        self.scenario_is_new = false;
        result
    }

    pub fn bbox(&self) -> Option<BoundingBox> {
        self.trajectory_buffer.as_ref().and_then(|buf| {
            let mut positions = buf.iter();
            positions.next().map(|first_position| {
                let mut bbox = BoundingBox::new_at(*first_position);
                positions.for_each(|position| bbox.expand_to(*position));
                bbox
            })
        })
    }

    pub fn visible_units(&self) -> f32 {
        self.visible_units
    }

    pub fn focus(&self) -> Point3 {
        self.focus
    }

    pub fn set_viewport(&mut self, focus: Point3, visible_units: f32) {
        log::debug!("canvas.set_viewport({},{})", focus, visible_units);
        self.focus = focus;
        self.visible_units = visible_units;
    }

    pub fn set_visible_bbox(&mut self, bbox: &BoundingBox) {
        self.set_viewport(bbox.center(), bbox.diameter() * 1.2);
    }

    pub fn adjust_scale_and_center(&mut self, paint_area: &egui::Rect) {
        let scale = f32::min(paint_area.width(), paint_area.height()) / self.visible_units;
        self.scale = Vec3::new(scale, -scale, 1.);
        self.area_center = paint_area.center();
    }

    pub fn user_to_screen(&self, pos: impl Into<Point3>) -> Pos2 {
        (pos.into() - self.focus)
            .component_mul(&self.scale)
            .to_pos2()
            + self.area_center.to_vec2()
    }

    pub fn screen_to_user(&self, pos: Pos2) -> Point3 {
        debug_assert!(self.scale != Vec3::default());
        self.focus
            + (pos - self.area_center.to_vec2())
                .to_vec3()
                .component_div(&self.scale)
    }

    pub fn update_model(&mut self, world: &World) {
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = ::std::time::Instant::now();

        let mut updated = false;
        let min_dt = self
            .integrations
            .iter()
            .map(|integration| integration.borrow().fetch_step_duration(world))
            .min() // this crate depends on R32 just to be able to use this min() function
            .unwrap_or_else(|| 0.1.into());

        let scenario_is_new = self.scenario_is_new_once(); //todo: method not needed
        let scenario = world.scenarios()[self.scenario_idx()].borrow();
        self.update_trajectory(&scenario, min_dt);
        self.integrations.iter().for_each(|integration| {
            let mut integration = integration.borrow_mut();
            if scenario_is_new {
                integration.reset();
            }
            let integrator = &*world[integration.integrator_idx()].borrow().core;
            let step_duration = world[integration.step_size_idx()].borrow().duration;
            updated |= integration.update(&scenario, integrator, step_duration);
        });

        #[cfg(not(target_arch = "wasm32"))]
        if updated {
            log::debug!(
                "Render Canvas: integrate: {}µs",
                start_time.elapsed().as_micros()
            );
        }

        if scenario_is_new {
            log::debug!("updating bounding box");
            self.update_bounding_box();
        }
    }

    pub fn update_bounding_box(&mut self) {
        if let Some(mut bbox) = self.bbox() {
            self.integrations()
                .for_each(|integration| integration.borrow().stretch_bbox(&mut bbox));
            self.set_visible_bbox(&bbox);
        }
    }

    pub fn check_references(&self, world: &World) -> Result<(), String> {
        self.scenario
            .check_reference(world.scenarios())
            .map_err(|err| format!("scenario: {}", err))?;
        for (n, integration) in self.integrations.iter().enumerate() {
            integration
                .borrow()
                .check_references(world)
                .map_err(|err| format!("integration #{}: {}", n + 1, err))?;
        }
        Ok(())
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
