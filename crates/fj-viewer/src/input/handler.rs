use fj_math::{Point, Scalar, Transform, Vector};

use crate::{
    camera::{Camera, FocusPoint},
    NormalizedScreenPosition,
};

use super::{rotation::Rotation, zoom, InputEvent};

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
                Rotation::apply(angle_x, angle_y, focus_point, camera);
            }
            InputEvent::Zoom(zoom_delta) => {
                zoom::apply_zoom(zoom_delta, focus_point, camera);
            }
        }
    }
}

pub fn apply_translation(
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
