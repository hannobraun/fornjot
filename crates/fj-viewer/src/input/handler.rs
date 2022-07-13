use super::{movement::Movement, rotation::Rotation, zoom::Zoom, Event};
use crate::camera::{Camera, FocusPoint};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
pub struct Handler {
    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    /// Handle an input event
    pub fn handle_event(
        &mut self,
        event: Event,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        match event {
            Event::Translate { previous, current } => {
                self.movement.apply(previous, current, focus_point, camera)
            }
            Event::Rotation { angle_x, angle_y } => {
                self.rotation.apply(angle_x, angle_y, focus_point, camera)
            }
            Event::Zoom(zoom_delta) => {
                self.zoom.apply(zoom_delta, focus_point, camera)
            }
        }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            movement: Movement,
            rotation: Rotation,
            zoom: Zoom,
        }
    }
}
