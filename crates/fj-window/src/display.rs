use fj_interop::Model;
use fj_viewer::{
    InputEvent, NormalizedScreenPosition, RendererInitError, Screen,
    ScreenSize, Viewer,
};
use futures::executor::block_on;
use winit::{
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{
        ElementState, Event, KeyEvent, MouseButton, MouseScrollDelta,
        WindowEvent,
    },
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
};

use crate::window::{self, Window};

/// Display the provided mesh in a window that processes input
pub fn display(model: Model, invert_zoom: bool) -> Result<(), Error> {
    let event_loop = EventLoop::new()?;
    let window = Window::new(&event_loop)?;
    let mut viewer = block_on(Viewer::new(&window))?;

    let mut display_state = DisplayState {
        held_mouse_button: None,
        new_size: None,
        stop_drawing: false,
    };

    viewer.handle_model_update(model);

    #[allow(deprecated)] // only for the transition to winit 0.30
    event_loop.run(move |event, event_loop_window_target| {
        let input_event = input_event(
            &event,
            &window,
            &display_state.held_mouse_button,
            viewer.cursor(),
            invert_zoom,
        );
        if let Some(input_event) = input_event {
            viewer.handle_input_event(input_event);
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                event_loop_window_target.exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => {
                    event_loop_window_target.exit();
                }
                Key::Character("1") => {
                    viewer.toggle_draw_model();
                }
                Key::Character("2") => {
                    viewer.toggle_draw_mesh();
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                display_state.new_size = Some(ScreenSize {
                    width: size.width,
                    height: size.height,
                });
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => match state {
                ElementState::Pressed => {
                    display_state.held_mouse_button = Some(button);
                    viewer.add_focus_point();
                }
                ElementState::Released => {
                    display_state.held_mouse_button = None;
                    viewer.remove_focus_point();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { .. },
                ..
            } => viewer.add_focus_point(),
            Event::AboutToWait => {
                window.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Only do a screen resize once per frame. This protects against
                // spurious resize events that cause issues with the renderer.
                if let Some(size) = display_state.new_size.take() {
                    display_state.stop_drawing =
                        size.width == 0 || size.height == 0;
                    if !display_state.stop_drawing {
                        viewer.handle_screen_resize(size);
                    }
                }

                if !display_state.stop_drawing {
                    viewer.draw();
                }
            }
            _ => {}
        }
    })?;

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
    held_mouse_button: Option<MouseButton>,
    new_size: Option<ScreenSize>,
    stop_drawing: bool,
}

fn input_event<T>(
    event: &Event<T>,
    window: &Window,
    held_mouse_button: &Option<MouseButton>,
    previous_cursor: &mut Option<NormalizedScreenPosition>,
    invert_zoom: bool,
) -> Option<InputEvent> {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
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
        Event::WindowEvent {
            event: WindowEvent::MouseWheel { delta, .. },
            ..
        } => {
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
