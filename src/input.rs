use winit::{
    event::{KeyboardInput, VirtualKeyCode},
    event_loop::ControlFlow,
};

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
}
