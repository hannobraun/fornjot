use nalgebra::{Point, Rotation3, Translation, Vector};

use crate::camera::{Camera, FocusPoint};

pub struct Rotation {
    active: bool,
    focus_point: FocusPoint,
}

impl Rotation {
    pub fn new() -> Self {
        Self {
            active: false,
            focus_point: FocusPoint::none(),
        }
    }

    pub fn start(&mut self, focus_point: FocusPoint) {
        self.active = true;
        self.focus_point = focus_point;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn apply(&self, diff_x: f64, diff_y: f64, camera: &mut Camera) {
        if self.active {
            let rotate_around =
                self.focus_point.0.unwrap_or_else(Point::origin);

            let f = 0.005;

            let angle_x = diff_y * f;
            let angle_y = diff_x * f;

            let trans = Translation::from(rotate_around);

            let rot_x = Rotation3::from_axis_angle(&Vector::x_axis(), angle_x);
            let rot_y = Rotation3::from_axis_angle(&Vector::y_axis(), angle_y);

            camera.rotation =
                trans * rot_y * rot_x * trans.inverse() * camera.rotation;
        }
    }
}
