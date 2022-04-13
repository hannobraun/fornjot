use nalgebra::{Rotation3, Translation, Vector};

use crate::camera::{Camera, FocusPoint};

pub struct Rotation {
    focus_point: FocusPoint,
}

impl Rotation {
    pub fn new() -> Self {
        Self {
            focus_point: FocusPoint::none(),
        }
    }

    pub fn start(&mut self, focus_point: FocusPoint) {
        self.focus_point = focus_point;
    }

    pub fn stop(&mut self) {
        self.focus_point = FocusPoint::none();
    }

    pub fn apply(&self, diff_x: f64, diff_y: f64, camera: &mut Camera) {
        if let Some(focus_point) = self.focus_point.0 {
            let f = 0.005;

            let angle_x = diff_y * f;
            let angle_y = diff_x * f;

            let trans = Translation::from(focus_point.coords);

            let rot_x = Rotation3::from_axis_angle(&Vector::x_axis(), angle_x);
            let rot_y = Rotation3::from_axis_angle(&Vector::y_axis(), angle_y);

            camera.rotation =
                trans * rot_y * rot_x * trans.inverse() * camera.rotation;
        }
    }
}
