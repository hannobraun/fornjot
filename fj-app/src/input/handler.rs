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
};

use super::{movement::Movement, rotation::Rotation, zoom::Zoom};

pub struct Handler {
    cursor: Option<PhysicalPosition<f64>>,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    pub fn new(now: Instant) -> Self {
        Self {
            cursor: None,

            movement: Movement::new(),
            rotation: Rotation::new(),
            zoom: Zoom::new(now),
        }
    }

    pub fn cursor(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor
    }

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

    pub fn handle_cursor_moved(
        &mut self,
        cursor: PhysicalPosition<f64>,
        camera: &mut Camera,
        window: &Window,
    ) {
        if let Some(previous) = self.cursor {
            let diff_x = cursor.x - previous.x;
            let diff_y = cursor.y - previous.y;

            self.movement.apply(self.cursor, camera, window);
            self.rotation.apply(diff_x, diff_y, camera);
        }

        self.cursor = Some(cursor);
    }

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

    pub fn update(
        &mut self,
        delta_t: f64,
        now: Instant,
        camera: &mut Camera,
        window: &Window,
        triangles: &Mesh<Point<3>>,
    ) {
        let focus_point = camera.focus_point(window, self.cursor, triangles);

        self.zoom.discard_old_events(now);
        self.zoom.update_speed(now, delta_t, focus_point, camera);

        camera.translation.z -= self.zoom.speed();
    }
}

pub struct Actions {
    pub exit: bool,

    pub toggle_model: bool,
    pub toggle_mesh: bool,
    pub toggle_debug: bool,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            exit: false,

            toggle_model: false,
            toggle_mesh: false,
            toggle_debug: false,
        }
    }
}
