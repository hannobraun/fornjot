use std::{collections::BTreeMap, thread};

use fj_interop::TriMesh;
use futures::executor::block_on;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{
        ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
    },
    event_loop::{ActiveEventLoop, EventLoop, EventLoopClosed, EventLoopProxy},
    keyboard::{Key, NamedKey},
    window::WindowId,
};

use crate::{
    RendererInitError,
    input::{DEFAULT_CAMERA_TUNING_CONFIG, InputEvent},
    window::Window,
};

/// # Create a model viewer and spawn a new thread where to use it
///
/// Create a model viewer that runs on the main thread, blocking the thread that
/// calls this function. Makes a handle to the viewer available to the provided
/// closure, allowing it to display models.
///
/// This API is a bit weird, due to the realities of native graphics on various
/// platforms. Those tend to insist on running on the main thread, so this
/// function spawns a new thread for the caller.
///
/// This function should be called from the application's main thread, or
/// displaying models might end up not working correctly.
pub fn make_viewer_and_spawn_thread(
    f: impl FnOnce(Viewer) + Send + 'static,
) -> Result<(), Error> {
    let mut builder = EventLoop::with_user_event();
    let event_loop = builder.build()?;

    let mut display_state = DisplayState {
        windows: BTreeMap::new(),
    };

    let proxy = event_loop.create_proxy();
    thread::spawn(|| f(Viewer { event_loop: proxy }));

    event_loop.run_app(&mut display_state)?;

    Ok(())
}

/// # Fornjot model viewer
pub struct Viewer {
    event_loop: EventLoopProxy<TriMesh>,
}

impl Viewer {
    /// # Display a triangle mesh in a new window
    ///
    /// This can fail, if the viewer thread is no longer running. Returns the
    /// triangle mesh, wrapped in an error, if that is the case.
    pub fn display(
        &self,
        tri_mesh: TriMesh,
    ) -> Result<(), EventLoopClosed<TriMesh>> {
        self.event_loop.send_event(tri_mesh)
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
    windows: BTreeMap<WindowId, Window>,
}

impl ApplicationHandler<TriMesh> for DisplayState {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.windows.get_mut(&window_id) else {
            return;
        };

        let input_event = input_event(&event);
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
            window.winit_window().request_redraw();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, tri_mesh: TriMesh) {
        let window = block_on(Window::new(tri_mesh, event_loop)).unwrap();
        self.windows.insert(window.winit_window().id(), window);
    }
}

fn input_event(event: &WindowEvent) -> Option<InputEvent> {
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

            Some(InputEvent::Zoom(delta))
        }
        _ => None,
    }
}
