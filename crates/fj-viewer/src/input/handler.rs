use super::{movement::Movement, rotation::Rotation, zoom::Zoom, InputEvent};
use crate::camera::{Camera, FocusPoint};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
pub struct InputHandler {
    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl InputHandler {
    /// Handle an input event
    pub fn handle_event(
        &mut self,
        event: InputEvent,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        match event {
            InputEvent::Translation { previous, current } => {
                self.movement.apply(previous, current, focus_point, camera)
            }
            InputEvent::Rotation { angle_x, angle_y } => {
                self.rotation.apply(angle_x, angle_y, focus_point, camera)
            }
            InputEvent::Zoom(zoom_delta) => {
                self.zoom.apply(zoom_delta, focus_point, camera)
            }
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            movement: Movement,
            rotation: Rotation,
            zoom: Zoom,
        }
    }
}
