use euclid::Angle;
use winit::{
    dpi::{LogicalPosition, PhysicalPosition},
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
    event_loop::ControlFlow,
};

use crate::transform::Transform;

pub struct InputHandler {
    cursor: Option<PhysicalPosition<f64>>,
    rotating: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            cursor: None,
            rotating: false,
        }
    }

    pub fn handle_keyboard_input(
        &mut self,
        input: KeyboardInput,
        control_flow: &mut ControlFlow,
    ) {
        match input {
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
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

                let x_angle = Angle::radians(diff_y as f32 * f);
                let y_angle = Angle::radians(diff_x as f32 * f);

                transform.rotation = transform
                    .rotation
                    .then_rotate(1.0, 0.0, 0.0, x_angle)
                    .then_rotate(0.0, 1.0, 0.0, y_angle);
            }
        }

        self.cursor = Some(position);
    }

    pub fn handle_mouse_input(
        &mut self,
        state: ElementState,
        button: MouseButton,
    ) {
        match state {
            ElementState::Pressed => {
                if button == MouseButton::Left {
                    self.rotating = true;
                }
            }
            ElementState::Released => {
                if button == MouseButton::Left {
                    self.rotating = false;
                }
            }
        }
    }

    pub fn handle_mouse_wheel(
        &mut self,
        delta: MouseScrollDelta,
        transform: &mut Transform,
    ) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y * 0.5,
            MouseScrollDelta::PixelDelta(LogicalPosition { y, .. }) => {
                y as f32 * 0.1
            }
        };

        transform.distance += delta;
    }
}
