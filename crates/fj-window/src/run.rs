//! Model viewer initialization and event processing
//!
//! Provides the functionality to create a window and perform basic viewing
//! with programmed models.

use std::error;

use fj_host::Watcher;
use fj_interop::status_report::StatusReport;
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{
    camera::Camera,
    graphics::{DrawConfig, Renderer, RendererInitError},
    input::{InputEvent, InputHandler},
    screen::{NormalizedScreenPosition, Screen as _, ScreenSize},
};
use futures::executor::block_on;
use tracing::{trace, warn};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::{self, Window};

/// Initializes a model viewer for a given model and enters its process loop.
pub fn run(
    watcher: Option<Watcher>,
    shape_processor: ShapeProcessor,
    mut status: StatusReport,
    invert_zoom: bool,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let mut previous_cursor = None;
    let mut held_mouse_button = None;
    let mut focus_point = None;

    let mut input_handler = InputHandler::default();
    let mut renderer = block_on(Renderer::new(&window))?;
    let mut egui_winit_state = egui_winit::State::new(&event_loop);

    let mut draw_config = DrawConfig::default();

    let mut shape = None;
    let mut camera = Camera::new();

    // Only handle resize events once every frame. This filters out spurious
    // resize events that can lead to wgpu warnings. See this issue for some
    // context:
    // https://github.com/rust-windowing/winit/issues/2094
    let mut new_size = None;

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        if let Some(watcher) = &watcher {
            if let Some(new_shape) = watcher.receive(&mut status) {
                match shape_processor.process(&new_shape) {
                    Ok(new_shape) => {
                        renderer.update_geometry(
                            (&new_shape.mesh).into(),
                            (&new_shape.debug_info).into(),
                            new_shape.aabb,
                        );

                        camera.update_planes(&new_shape.aabb);
                        shape = Some(new_shape);
                    }
                    Err(err) => {
                        // Can be cleaned up, once `Report` is stable:
                        // https://doc.rust-lang.org/std/error/struct.Report.html

                        println!("Shape processing error: {}", err);

                        let mut current_err = &err as &dyn error::Error;
                        while let Some(err) = current_err.source() {
                            println!();
                            println!("Caused by:");
                            println!("    {}", err);

                            current_err = err;
                        }
                    }
                }
            }
        }

        if let Event::WindowEvent {
            event: window_event,
            ..
        } = &event
        {
            // In theory we could/should check if `egui` wants "exclusive" use
            // of this event here. But with the current integration with Fornjot
            // we're kinda blurring the lines between "app" and "platform", so
            // for the moment we pass every event to both `egui` & Fornjot.
            //
            // The primary visible impact of this currently is that if you drag
            // a title bar that overlaps the model then both the model & window
            // get moved.
            egui_winit_state.on_event(&renderer.gui.context, window_event);
        }

        // fj-window events
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(virtual_key_code),
                                ..
                            },
                        ..
                    },
                ..
            } => match virtual_key_code {
                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                VirtualKeyCode::Key1 => {
                    draw_config.draw_model = !draw_config.draw_model
                }
                VirtualKeyCode::Key2 => {
                    if renderer.is_line_drawing_available() {
                        draw_config.draw_mesh = !draw_config.draw_mesh
                    }
                }
                VirtualKeyCode::Key3 => {
                    if renderer.is_line_drawing_available() {
                        draw_config.draw_debug = !draw_config.draw_debug
                    }
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                new_size = Some(ScreenSize {
                    width: size.width,
                    height: size.height,
                });
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                match state {
                    ElementState::Pressed => held_mouse_button = Some(button),
                    ElementState::Released => held_mouse_button = None,
                };
            }
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let Some(size) = new_size.take() {
                    renderer.handle_resize(size);
                }

                let egui_input =
                    egui_winit_state.take_egui_input(window.window());

                if let Err(err) = renderer.draw(
                    &camera,
                    &mut draw_config,
                    window.window().scale_factor() as f32,
                    &mut status,
                    egui_input,
                ) {
                    warn!("Draw error: {}", err);
                }
            }
            _ => {}
        }

        if let (Some(shape), Some(should_focus)) = (&shape, focus_event(&event))
        {
            if should_focus {
                // Don't unnecessarily recalculate focus point
                if focus_point.is_none() {
                    focus_point =
                        Some(camera.focus_point(previous_cursor, shape));
                }
            } else {
                focus_point = None;
            }
        }

        let input_event = input_event(
            &event,
            &window,
            &held_mouse_button,
            &mut previous_cursor,
            invert_zoom,
        );
        if let (Some(input_event), Some(focus_point)) =
            (input_event, focus_point)
        {
            input_handler.handle_event(input_event, focus_point, &mut camera);
        }
    });
}

fn input_event(
    event: &Event<()>,
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
                    (*y as f64) * ZOOM_FACTOR_LINE
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

/// Returns true/false if focus point point should be created/removed
/// None means no change to focus point is needed
fn focus_event(event: &Event<()>) -> Option<bool> {
    match event {
        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    state,
                    button: MouseButton::Left | MouseButton::Right,
                    ..
                },
            ..
        } => match state {
            ElementState::Pressed => Some(true),
            ElementState::Released => Some(false),
        },
        Event::WindowEvent {
            event: WindowEvent::MouseWheel { .. },
            ..
        } => Some(true),
        _ => None,
    }
}

/// Error in main loop
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing window
    #[error("Error initializing window")]
    WindowInit(#[from] window::Error),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    GraphicsInit(#[from] RendererInitError),
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
