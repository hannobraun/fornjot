use fj_math::{Point, Scalar, Transform, Vector};

use crate::{
    camera::{Camera, FocusPoint},
    NormalizedScreenPosition,
};

use super::InputEvent;

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
#[derive(Default)]
pub struct InputHandler;

impl InputHandler {
    /// Handle an input event
    pub fn handle_event(
        event: InputEvent,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        match event {
            InputEvent::Translation { previous, current } => {
                apply_translation(previous, current, focus_point, camera);
            }
            InputEvent::Rotation { angle_x, angle_y } => {
                apply_rotation(angle_x, angle_y, focus_point, camera);
            }
            InputEvent::Zoom(zoom_delta) => {
                apply_zoom(zoom_delta, focus_point, camera);
            }
        }
    }
}

fn apply_translation(
    previous: NormalizedScreenPosition,
    current: NormalizedScreenPosition,
    focus_point: FocusPoint,
    camera: &mut Camera,
) {
    let previous = camera.cursor_to_model_space(previous);
    let cursor = camera.cursor_to_model_space(current);

    let d1 = Point::distance_to(&camera.position(), &cursor);
    let d2 = Point::distance_to(&camera.position(), &focus_point.0);

    let diff = (cursor - previous) * d2 / d1;
    let offset = camera.camera_to_model().transform_vector(&diff);

    camera.translation = camera.translation
        * Transform::translation(Vector::from([
            offset.x,
            offset.y,
            Scalar::ZERO,
        ]));
}

fn apply_rotation(
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

fn apply_zoom(zoom_delta: f64, focus_point: FocusPoint, camera: &mut Camera) {
    let distance = (focus_point.0 - camera.position()).magnitude();
    let displacement = zoom_delta * distance.into_f64();
    camera.translation = camera.translation
        * Transform::translation(Vector::from([0.0, 0.0, displacement]));
}
