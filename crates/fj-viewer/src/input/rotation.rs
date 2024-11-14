use fj_math::Transform;

use crate::camera::{Camera, FocusPoint};

pub struct Rotation;

impl Rotation {
    pub fn apply(
        angle_x: f64,
        angle_y: f64,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        let rotate_around = Transform::translation(focus_point.0.coords);

        // the model rotates not the camera, so invert the transform
        let camera_rotation = camera.rotation.inverse();

        let rotation = Transform::rotation(camera_rotation.right() * angle_x)
            * Transform::rotation(camera_rotation.up() * angle_y);

        let transform = camera.camera_to_model()
            * rotate_around
            * rotation
            * rotate_around.inverse();

        camera.rotation = transform.extract_rotation();
        camera.translation = transform.extract_translation();
    }
}
