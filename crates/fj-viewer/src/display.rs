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
    viewer::ViewerWindow,
    window::{self, ScreenSize, Window},
};

/// # Display the provided mesh in a window that processes input
pub fn display(tri_mesh: TriMesh, invert_zoom: bool) -> Result<(), Error> {
    let event_loop = EventLoop::new()?;

    let mut display_state = DisplayState {
        tri_mesh: Some(tri_mesh),
        invert_zoom,
        window: None,
    };

    event_loop.run_app(&mut display_state)?;

    Ok(())
}

/// Main loop initialization error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing event loop
    #[error("Error initializing event loop")]
    EventLoop(#[from] EventLoopError),

    /// Error initializing window
    #[error("Error initializing window")]
    Window(#[from] window::WindowError),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    Graphics(#[from] RendererInitError),
}

struct DisplayState {
    tri_mesh: Option<TriMesh>,
    invert_zoom: bool,
    window: Option<ViewerWindow>,
}

impl ApplicationHandler for DisplayState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = self.window.get_or_insert_with(|| {
            let window = Window::new(event_loop).unwrap();
            block_on(ViewerWindow::new(window)).unwrap()
        });

        if let Some(mesh) = self.tri_mesh.take() {
            window.handle_model_update(mesh);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &mut self.window else {
            return;
        };

        let input_event = input_event(&event, self.invert_zoom);
        if let Some(input_event) = input_event {
            window.handle_input_event(input_event);
        }

        match event {
            WindowEvent::Resized(size) => {
                window.on_screen_resize(ScreenSize {
                    width: size.width,
                    height: size.height,
                });
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
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let Some(window) = &self.window else { return };
        window.window().winit_window().request_redraw();
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
