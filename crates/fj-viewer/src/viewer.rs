use std::{collections::BTreeMap, panic, thread};

use fj_interop::TriMesh;
use futures::executor::block_on;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{
        ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
    },
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{Key, NamedKey},
    window::WindowId,
};

use crate::{
    RendererInitError,
    input::DEFAULT_CAMERA_TUNING_CONFIG,
    window::{ToDisplay, Window},
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
pub fn make_viewer_and_spawn_thread<R>(
    f: impl FnOnce(Viewer) -> R + Send + 'static,
) -> Result<R, Error>
where
    R: Send + 'static,
{
    let event_loop = EventLoop::with_user_event().build()?;

    let mut display_state = DisplayState {
        windows: BTreeMap::new(),
    };

    let proxy = event_loop.create_proxy();
    let handle = thread::spawn(|| f(Viewer { event_loop: proxy }));

    event_loop.run_app(&mut display_state)?;

    let result = match handle.join() {
        Ok(result) => result,
        Err(payload) => panic::resume_unwind(payload),
    };

    Ok(result)
}

/// # Fornjot model viewer
pub struct Viewer {
    event_loop: EventLoopProxy<EventLoopEvent>,
}

impl Viewer {
    /// # Display a triangle mesh in a new window
    pub fn display_mesh(&self, tri_mesh: TriMesh) {
        // If there's an error, that means the display thread has closed down
        // and we're on our way to shutting down as well. I don't think there's
        // much we can do about that.
        let _ = self
            .event_loop
            .send_event(EventLoopEvent::DisplayMesh { tri_mesh });
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

enum EventLoopEvent {
    DisplayMesh { tri_mesh: TriMesh },
}

struct DisplayState {
    windows: BTreeMap<WindowId, Window>,
}

impl ApplicationHandler<EventLoopEvent> for DisplayState {
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
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        f64::from(y)
                            * DEFAULT_CAMERA_TUNING_CONFIG.zoom_sensitivity_line
                    }
                    MouseScrollDelta::PixelDelta(PhysicalPosition {
                        y,
                        ..
                    }) => {
                        y * DEFAULT_CAMERA_TUNING_CONFIG.zoom_sensitivity_pixel
                    }
                };

                window.add_focus_point();
                window.on_zoom(delta);
            }
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

    fn user_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: EventLoopEvent,
    ) {
        let EventLoopEvent::DisplayMesh { tri_mesh } = event;
        let to_display = ToDisplay::model(tri_mesh);

        let window = block_on(Window::new(to_display, event_loop)).unwrap();
        self.windows.insert(window.winit_window().id(), window);
    }
}
