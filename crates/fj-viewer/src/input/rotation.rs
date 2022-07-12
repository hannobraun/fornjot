use fj_math::{Point, Transform, Vector};

use crate::{
    camera::{Camera, FocusPoint},
    screen::NormalizedPosition,
};

pub struct Rotation;

impl Rotation {
    pub fn apply(
        &self,
        previous: NormalizedPosition,
        current: NormalizedPosition,
        focus_point: &FocusPoint,
        camera: &mut Camera,
    ) {
        let rotate_around: Vector<3> =
            focus_point.0.unwrap_or_else(Point::origin).coords;

        let f = -5.;

        let diff_x = current.x - previous.x;
        let diff_y = current.y - previous.y;
        let angle_x = diff_y * f;
        let angle_y = -diff_x * f;

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

fn up_vector(rotation: &Transform) -> Vector<3> {
    let d = rotation.data();
    Vector::from_components_f64([d[4], d[5], d[6]])
}

fn right_vector(rotation: &Transform) -> Vector<3> {
    let d = rotation.data();
    Vector::from_components_f64([d[0], d[1], d[2]])
}
