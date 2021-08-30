use nalgebra::{Rotation3, Unit};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
};

use crate::graphics::Transform;

pub struct Handler {
    cursor: Option<PhysicalPosition<f64>>,
    rotating: bool,
    moving: bool,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            cursor: None,
            rotating: false,
            moving: false,
        }
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
                _ => (),
            }
        }
    }

    pub fn handle_cursor_moved(
        &mut self,
        position: PhysicalPosition<f64>,
        transform: &mut Transform,
    ) {
        if let Some(previous) = self.cursor {
            let diff_x = position.x - previous.x;
            let diff_y = position.y - previous.y;

            if self.rotating {
                let f = 0.005;

                let x_angle = diff_y as f32 * f;
                let y_angle = diff_x as f32 * f;

                let x_rot = Rotation3::from_axis_angle(
                    &Unit::new_unchecked([1.0, 0.0, 0.0].into()),
                    x_angle,
                );
                let y_rot = Rotation3::from_axis_angle(
                    &Unit::new_unchecked([0.0, 1.0, 0.0].into()),
                    y_angle,
                );

                transform.rotation = y_rot * x_rot * transform.rotation;
            }
            if self.moving {
                // TASK: Implement.
                println!("Moving...");
            }
        }

        self.cursor = Some(position);
    }

    pub fn handle_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
    ) {
        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                self.rotating = true;
            }
            (MouseButton::Left, ElementState::Released) => {
                self.rotating = false;
            }
            (MouseButton::Right, ElementState::Pressed) => {
                self.moving = true;
            }
            (MouseButton::Right, ElementState::Released) => {
                self.moving = false;
            }
            _ => {}
        }
    }

    pub fn handle_mouse_wheel(
        &mut self,
        delta: MouseScrollDelta,
        transform: &mut Transform,
    ) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y * 50.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                y as f32 * 5.0
            }
        };

        transform.distance += delta;
    }
}

pub struct Actions {
    pub exit: bool,

    pub toggle_model: bool,
    pub toggle_mesh: bool,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            exit: false,

            toggle_model: false,
            toggle_mesh: false,
        }
    }
}
