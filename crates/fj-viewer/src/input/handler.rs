use super::{movement::Movement, rotation::Rotation, zoom::Zoom, Event};
use crate::camera::{Camera, FocusPoint};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
pub struct Handler {
    focus_point: FocusPoint,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    /// Handle an input event
    pub fn handle_event(&mut self, event: Event, camera: &mut Camera) {
        match event {
            Event::Translate { previous, current } => self.movement.apply(
                previous,
                current,
                &self.focus_point,
                camera,
            ),
            Event::Rotation { angle_x, angle_y } => {
                self.rotation
                    .apply(angle_x, angle_y, &self.focus_point, camera)
            }
            Event::Zoom(zoom_delta) => {
                self.zoom.apply(zoom_delta, &self.focus_point, camera)
            }
        }
    }

    /// A new focus point was selected (or deselected)
    pub fn focus(&mut self, focus_point: FocusPoint) {
        self.focus_point = focus_point;
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            focus_point: FocusPoint::none(),

            movement: Movement,
            rotation: Rotation,
            zoom: Zoom,
        }
    }
}
