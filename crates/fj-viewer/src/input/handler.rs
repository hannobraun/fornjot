use std::time::Instant;

use fj_interop::mesh::Mesh;
use fj_math::Point;
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
};

use crate::{
    camera::{Camera, FocusPoint},
    window::Window,
    Winit,
};

use super::{movement::Movement, rotation::Rotation, zoom::Zoom};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
///
#[doc = include_str!("../../docs/phantom_type.md")]
pub struct Handler<T> {
    cursor: Option<PhysicalPosition<f64>>,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,

    backend: std::marker::PhantomData<T>,
}

impl Handler<Winit> {
    /// Returns a new Handler.
    ///
    /// # Examples
    /// ```rust no_run
    /// // Store initialization time for camera zoom calculations
    /// let instant = std::time::Instant::now();
    /// let input_handler = fj_viewer::input::Handler::new(instant);
    /// ```
    pub fn new(now: Instant) -> Self {
        Self {
            cursor: None,

            movement: Movement::new(),
            rotation: Rotation::new(),
            zoom: Zoom::new(now),

            backend: std::marker::PhantomData,
        }
    }

    /// Returns the state of the cursor position.
    pub fn cursor(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor
    }

    /// Applies user input to `actions`.
    pub fn handle_keyboard_input(
        &mut self,
        input: KeyboardInput,
        actions: &mut Actions,
    ) {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(virtual_key_code),
            ..
        } = input
        {
            match virtual_key_code {
                VirtualKeyCode::Escape => actions.exit = true,

                VirtualKeyCode::Key1 => actions.toggle_model = true,
                VirtualKeyCode::Key2 => actions.toggle_mesh = true,
                VirtualKeyCode::Key3 => actions.toggle_debug = true,

                _ => (),
            }
        }
    }

    /// Applies cursor movement to `camera`.
    pub fn handle_cursor_moved(
        &mut self,
        cursor: PhysicalPosition<f64>,
        camera: &mut Camera,
        window: &Window<winit::window::Window>,
    ) {
        if let Some(previous) = self.cursor {
            let diff_x = cursor.x - previous.x;
            let diff_y = cursor.y - previous.y;

            self.movement.apply(self.cursor, camera, window);
            self.rotation.apply(diff_x, diff_y, camera);
        }

        self.cursor = Some(cursor);
    }

    /// Updates `state` and `focus_point` when mouse is clickled.
    pub fn handle_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
        focus_point: FocusPoint,
    ) {
        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                self.rotation.start(focus_point);
            }
            (MouseButton::Left, ElementState::Released) => {
                self.rotation.stop();
            }
            (MouseButton::Right, ElementState::Pressed) => {
                self.movement.start(focus_point, self.cursor);
            }
            (MouseButton::Right, ElementState::Released) => {
                self.movement.stop();
            }
            _ => {}
        }
    }

    /// Updates zoom state from the scroll wheel.
    pub fn handle_mouse_wheel(
        &mut self,
        delta: MouseScrollDelta,
        now: Instant,
    ) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64 * 10.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y,
        };

        self.zoom.push_input_delta(delta, now);
    }

    /// Update application state from user input.
    pub fn update(
        &mut self,
        delta_t: f64,
        now: Instant,
        camera: &mut Camera,
        window: &Window<winit::window::Window>,
        mesh: &Mesh<Point<3>>,
    ) {
        let focus_point = camera.focus_point(window, self.cursor, mesh);

        self.zoom.discard_old_events(now);
        self.zoom.update_speed(now, delta_t, focus_point, camera);

        camera.translation.z -= self.zoom.speed();
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
