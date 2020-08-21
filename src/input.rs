use winit::{
    dpi::LogicalPosition,
    event::{KeyboardInput, MouseScrollDelta, VirtualKeyCode},
    event_loop::ControlFlow,
};

use crate::transform::Transform;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
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
