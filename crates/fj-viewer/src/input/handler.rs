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
            Event::FocusPoint(focus_point) => self.focus_point = focus_point,
            Event::Pan { previous, current } => self.movement.apply(
                previous,
                current,
                &self.focus_point,
                camera,
            ),
            Event::Orbit { previous, current } => self.rotation.apply(
                previous,
                current,
                &self.focus_point,
                camera,
            ),
            Event::Zoom(zoom_delta) => {
                self.zoom.apply(zoom_delta, &self.focus_point, camera)
            }
            _ => {}
        }
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
