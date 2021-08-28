use nalgebra::{Rotation3, Unit};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
    event_loop::ControlFlow,
};

use crate::graphics::Transform;

pub struct Handler {
    cursor: Option<PhysicalPosition<f64>>,
    rotating: bool,
}

impl Handler {
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
        if let KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } = input
        {
            *control_flow = ControlFlow::Exit;
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
            MouseScrollDelta::LineDelta(_, y) => y * 50.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                y as f32 * 5.0
            }
        };

        transform.distance += delta;
    }
}
