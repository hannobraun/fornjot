use fj_interop::Model;
use fj_viewer::{
    InputEvent, NormalizedScreenPosition, RendererInitError, Screen,
    ScreenSize, Viewer,
};
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

use crate::window::{self, Window};

/// Display the provided mesh in a window that processes input
pub fn display(model: Model, invert_zoom: bool) -> Result<(), Error> {
    let event_loop = EventLoop::new()?;

    let mut display_state = DisplayState {
        model: Some(model),
        invert_zoom,
        window: None,
        viewer: None,
        held_mouse_button: None,
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
    model: Option<Model>,
    invert_zoom: bool,
    window: Option<Window>,
    viewer: Option<Viewer>,
    held_mouse_button: Option<MouseButton>,
}

impl ApplicationHandler for DisplayState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = self
            .window
            .get_or_insert_with(|| Window::new(event_loop).unwrap());

        let viewer = self
            .viewer
            .get_or_insert_with(|| block_on(Viewer::new(window)).unwrap());

        if let Some(model) = self.model.take() {
            viewer.handle_model_update(model);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else { return };
        let Some(viewer) = &mut self.viewer else {
            return;
        };

        let input_event = input_event(
            &event,
            window,
            &self.held_mouse_button,
            viewer.cursor(),
            self.invert_zoom,
        );
        if let Some(input_event) = input_event {
            viewer.handle_input_event(input_event);
        }

        match event {
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
                    viewer.toggle_draw_model();
                }
                Key::Character("2") => {
                    viewer.toggle_draw_mesh();
                }
                _ => {}
            },
            WindowEvent::Resized(size) => {
                viewer.on_screen_resize(ScreenSize {
                    width: size.width,
                    height: size.height,
                });
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    self.held_mouse_button = Some(button);
                    viewer.add_focus_point();
                }
                ElementState::Released => {
                    self.held_mouse_button = None;
                    viewer.remove_focus_point();
                }
            },
            WindowEvent::MouseWheel { .. } => viewer.add_focus_point(),
            WindowEvent::RedrawRequested => {
                viewer.draw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let Some(window) = &self.window else { return };
        window.window().request_redraw();
    }
}

fn input_event(
    event: &WindowEvent,
    window: &Window,
    held_mouse_button: &Option<MouseButton>,
    previous_cursor: &mut Option<NormalizedScreenPosition>,
    invert_zoom: bool,
) -> Option<InputEvent> {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let [width, height] = window.size().as_f64();
            let aspect_ratio = width / height;

            // Cursor position in normalized coordinates (-1 to +1) with
            // aspect ratio taken into account.
            let current = NormalizedScreenPosition {
                x: position.x / width * 2. - 1.,
                y: -(position.y / height * 2. - 1.) / aspect_ratio,
            };
            let event = match (*previous_cursor, held_mouse_button) {
                (Some(previous), Some(button)) => match button {
                    MouseButton::Left => {
                        let diff_x = current.x - previous.x;
                        let diff_y = current.y - previous.y;
                        let angle_x = -diff_y * ROTATION_SENSITIVITY;
                        let angle_y = diff_x * ROTATION_SENSITIVITY;

                        Some(InputEvent::Rotation { angle_x, angle_y })
                    }
                    MouseButton::Right => {
                        Some(InputEvent::Translation { previous, current })
                    }
                    _ => None,
                },
                _ => None,
            };
            *previous_cursor = Some(current);
            event
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let delta = match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    f64::from(*y) * ZOOM_FACTOR_LINE
                }
                MouseScrollDelta::PixelDelta(PhysicalPosition {
                    y, ..
                }) => y * ZOOM_FACTOR_PIXEL,
            };

            let delta = if invert_zoom { -delta } else { delta };

            Some(InputEvent::Zoom(delta))
        }
        _ => None,
    }
}

/// Affects the speed of zoom movement given a scroll wheel input in lines.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_LINE: f64 = 0.075;

/// Affects the speed of zoom movement given a scroll wheel input in pixels.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_PIXEL: f64 = 0.005;

/// Affects the speed of rotation given a change in normalized screen position [-1, 1]
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ROTATION_SENSITIVITY: f64 = 5.;
