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

            let rotate_around = Transform::translation(rotate_around);

            // the model rotates not the camera, so invert the transform
            let camera_rotation = camera.rotation.inverse();
            let right_vector = right_vector(&camera_rotation);
            let up_vector = up_vector(&camera_rotation);

            let rotation = Transform::rotation(right_vector * angle_x)
                * Transform::rotation(up_vector * angle_y);

            let transform = camera.camera_to_model()
                * rotate_around
                * rotation
                * rotate_around.inverse();

            camera.rotation = transform.extract_rotation();
            camera.translation = transform.extract_translation();
        }
    }
}

fn up_vector(rotation: &Transform) -> Vector<3> {
    let d = rotation.data();
    Vector::from_components_f64([d[4], d[5], d[6]])
}

fn right_vector(rotation: &Transform) -> Vector<3> {
    let d = rotation.data();
    Vector::from_components_f64([d[0], d[1], d[2]])
}
