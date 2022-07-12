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
    pub fn handle_event(
        &mut self,
        event: Event,
        camera: &mut Camera,
        actions: &mut Actions,
    ) {
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
                self.zoom
                    .apply_to_camera(zoom_delta, &self.focus_point, camera)
            }
            Event::Exit => actions.exit = true,
            Event::ToggleModel => actions.toggle_model = true,
            Event::ToggleMesh => actions.toggle_mesh = true,
            Event::ToggleDebug => actions.toggle_debug = true,
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

/// Intermediate input state container
///
/// Used as a per frame state container for sending application state to `winit`.
#[derive(Default)]
pub struct Actions {
    /// Application exit state.
    pub exit: bool,

    /// Toggle for the shaded display of the model.
    pub toggle_model: bool,
    /// Toggle for the model's wireframe.
    pub toggle_mesh: bool,
    /// Toggle for debug information.
    pub toggle_debug: bool,
}

impl Actions {
    /// Returns a new `Actions`.
    pub fn new() -> Self {
        Self::default()
    }
}
