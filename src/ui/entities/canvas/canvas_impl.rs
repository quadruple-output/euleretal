use super::{
    core::{Duration, Obj, Scenario},
    import::{Point3, Vec3},
    misc::BoundingBox,
    trajectory_buffer::TrajectoryBuffer,
    ui_import::{egui, Pos2, Ui, Vec2},
    Integration, Painter,
};
use ::std::{cell::RefCell, rc::Rc};

pub struct Canvas {
    scenario: Obj<Scenario>,
    pub(super) integrations: Vec<Obj<Integration>>,
    pub(super) visible_units: f32,
    pub(super) focus: Point3,
    scale: Vec3,
    area_center: Pos2,
    pub(super) trajectory_buffer: Option<TrajectoryBuffer>,
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
            focus: Point3::origin(),
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

    pub fn integrations(&self) -> ::core::slice::Iter<Obj<Integration>> {
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

    pub fn has_trajectory(&self) -> bool {
        self.trajectory_buffer.is_some()
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

    pub fn set_visible_bbox(&mut self, bbox: &BoundingBox) {
        self.focus = bbox.center();
        self.visible_units = bbox.diameter() * 1.2;
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