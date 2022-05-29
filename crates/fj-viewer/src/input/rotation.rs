use fj_math::{Point, Transform, Vector};

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
            let rotate_around: Vector<3> =
                self.focus_point.0.unwrap_or_else(Point::origin).coords;

            let f = 0.005;

            let angle_x = diff_y * f;
            let angle_y = diff_x * f;

            let trans = Transform::translation(rotate_around);

            let aa_x = Vector::unit_x() * angle_x;
            let aa_y = Vector::unit_y() * angle_y;
            let rot_x = Transform::rotation(aa_x);
            let rot_y = Transform::rotation(aa_y);

            let inv = trans.inverse();

            camera.rotation = camera.rotation * trans * rot_y * rot_x * inv;
        }
    }
}
