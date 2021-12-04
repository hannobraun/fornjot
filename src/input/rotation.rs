use nalgebra::{Rotation3, Translation};

use crate::{
    camera::Camera,
    math::{Point, Vector},
};

pub struct Rotation {
    focus_point: Option<Point>,
}

impl Rotation {
    pub fn new() -> Self {
        Self { focus_point: None }
    }

    pub fn start(&mut self, focus_point: Option<Point>) {
        self.focus_point = focus_point;
    }

    pub fn stop(&mut self) {
        self.focus_point = None;
    }

    pub fn apply(&self, angle_x: f64, angle_y: f64, camera: &mut Camera) {
        if let Some(focus_point) = self.focus_point {
            let trans = Translation::from(focus_point.coords);

            let rot_x = Rotation3::from_axis_angle(&Vector::x_axis(), angle_x);
            let rot_y = Rotation3::from_axis_angle(&Vector::y_axis(), angle_y);

            camera.rotation =
                trans * rot_y * rot_x * trans.inverse() * camera.rotation;
        }
    }
}
