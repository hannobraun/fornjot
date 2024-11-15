use fj_math::{Transform, Vector};

use crate::camera::{Camera, FocusPoint};

pub fn apply_rotation(
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

pub fn apply_zoom(
    zoom_delta: f64,
    focus_point: FocusPoint,
    camera: &mut Camera,
) {
    let distance = (focus_point.0 - camera.position()).magnitude();
    let displacement = zoom_delta * distance.into_f64();
    camera.translation = camera.translation
        * Transform::translation(Vector::from([0.0, 0.0, displacement]));
}
