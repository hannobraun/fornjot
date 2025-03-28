use std::collections::BTreeMap;

use fj_interop::TriMesh;
use futures::executor::block_on;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{
        ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
    },
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowId,
};

use crate::{
    RendererInitError,
    input::{DEFAULT_CAMERA_TUNING_CONFIG, InputEvent},
    window::Window,
};

/// # Fornjot model viewer
pub struct Viewer {}

impl Viewer {
    /// # Display the provided mesh in a window that processes input
    pub fn new(tri_mesh: TriMesh, invert_zoom: bool) -> Result<Viewer, Error> {
        let event_loop = EventLoop::new()?;

        let mut display_state = DisplayState {
            tri_mesh: Some(tri_mesh),
            invert_zoom,
            window: BTreeMap::new(),
        };

        event_loop.run_app(&mut display_state)?;

        Ok(Viewer {})
    }
}

/// Main loop initialization error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing event loop
    #[error("Error initializing event loop")]
    EventLoop(#[from] EventLoopError),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    Graphics(#[from] RendererInitError),
}

struct DisplayState {
    tri_mesh: Option<TriMesh>,
    invert_zoom: bool,
    window: BTreeMap<WindowId, Window>,
}

impl ApplicationHandler for DisplayState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut entry = self.window.first_entry();
        let window = if let Some(window) = &mut entry {
            window.get_mut()
        } else {
            let window = block_on(Window::new(event_loop)).unwrap();
            self.window.entry(window.window().id()).or_insert(window)
        };

        if let Some(mesh) = self.tri_mesh.take() {
            window.handle_model_update(mesh);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.get_mut(&window_id) else {
            return;
        };

        let input_event = input_event(&event, self.invert_zoom);
        if let Some(input_event) = input_event {
            window.handle_input_event(input_event);
        }

        let mut drawn = false;

        match event {
            WindowEvent::Resized(size) => {
                window.on_screen_resize(size);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => {
                    event_loop.exit();
                }
                Key::Character("1") => {
                    window.toggle_draw_model();
                }
                Key::Character("2") => {
                    window.toggle_draw_mesh();
                }
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                window.on_cursor_movement([position.x, position.y]);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    MouseButton::Left => Some(crate::input::MouseButton::Left),
                    MouseButton::Right => {
                        Some(crate::input::MouseButton::Right)
                    }
                    _ => None,
                };

                if let Some(button) = button {
                    match state {
                        ElementState::Pressed => {
                            window.on_mouse_button_pressed(button);
                        }
                        ElementState::Released => {
                            window.on_mouse_button_released(button)
                        }
                    }
                }
            }
            WindowEvent::MouseWheel { .. } => window.add_focus_point(),
            WindowEvent::RedrawRequested => {
                window.draw();
                drawn = true;
            }
            _ => {}
        }

        if !drawn {
            window.window().request_redraw();
        }
    }
}

fn input_event(event: &WindowEvent, invert_zoom: bool) -> Option<InputEvent> {
    match event {
        WindowEvent::MouseWheel { delta, .. } => {
            let delta = match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    f64::from(*y)
                        * DEFAULT_CAMERA_TUNING_CONFIG.zoom_sensitivity_line
                }
                MouseScrollDelta::PixelDelta(PhysicalPosition {
                    y, ..
                }) => y * DEFAULT_CAMERA_TUNING_CONFIG.zoom_sensitivity_pixel,
            };

            let delta = if invert_zoom { -delta } else { delta };

            Some(InputEvent::Zoom(delta))
        }
        _ => None,
    }
}
