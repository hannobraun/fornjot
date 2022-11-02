//! Model viewer initialization and event processing
//!
//! Provides the functionality to create a window and perform basic viewing
//! with programmed models.

use std::error;

use fj_host::{Host, Model, ModelEvent, Parameters};
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{
    GuiState, InputEvent, NormalizedScreenPosition, RendererInitError, Screen,
    ScreenSize, StatusReport, Viewer,
};
use futures::executor::block_on;
use tracing::trace;
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
    model: Option<Model>,
    shape_processor: ShapeProcessor,
    invert_zoom: bool,
) -> Result<(), Error> {
    let mut status = StatusReport::new();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;
    let mut viewer = block_on(Viewer::new(&window))?;

    let mut held_mouse_button = None;

    let mut egui_winit_state = egui_winit::State::new(&event_loop);

    let mut host = model.map(Host::from_model).transpose()?;

    // Only handle resize events once every frame. This filters out spurious
    // resize events that can lead to wgpu warnings. See this issue for some
    // context:
    // https://github.com/rust-windowing/winit/issues/2094
    let mut new_size = None;

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        if let Some(host) = &host {
            loop {
                let events = host.events();
                let event = events
                    .try_recv()
                    .map_err(|err| {
                        if err.is_disconnected() {
                            panic!("Expected channel to never disconnect");
                        }
                    })
                    .ok();

                let event = match event {
                    Some(status_update) => status_update,
                    None => break,
                };

                match event {
                    ModelEvent::ChangeDetected => {
                        status.update_status(
                            "Change in model detected. Compiling...",
                        );
                    }
                    ModelEvent::Evaluation(evaluation) => {
                        status.update_status(&format!(
                            "Model compiled successfully in {}!",
                            evaluation.compile_time
                        ));

                        match shape_processor.process(&evaluation.shape) {
                            Ok(shape) => {
                                viewer.handle_shape_update(shape);
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

                    ModelEvent::Error(err) => {
                        // Can be cleaned up, once `Report` is stable:
                        // https://doc.rust-lang.org/std/error/struct.Report.html

                        println!("Error receiving updated shape: {}", err);

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

        if let Event::WindowEvent { event, .. } = &event {
            // In theory we could/should check if `egui` wants "exclusive" use
            // of this event here. But with the current integration with Fornjot
            // we're kinda blurring the lines between "app" and "platform", so
            // for the moment we pass every event to both `egui` & Fornjot.
            //
            // The primary visible impact of this currently is that if you drag
            // a title bar that overlaps the model then both the model & window
            // get moved.
            egui_winit_state.on_event(viewer.gui.context(), event);
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
                    viewer.toggle_draw_model();
                }
                VirtualKeyCode::Key2 => {
                    viewer.toggle_draw_mesh();
                }
                VirtualKeyCode::Key3 => {
                    viewer.toggle_draw_debug();
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
            } => match state {
                ElementState::Pressed => {
                    held_mouse_button = Some(button);
                    viewer.add_focus_point();
                }
                ElementState::Released => {
                    held_mouse_button = None;
                    viewer.remove_focus_point();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { .. },
                ..
            } => viewer.add_focus_point(),
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Only do a screen resize once per frame. This protects against
                // spurious resize events that cause issues with the renderer.
                if let Some(size) = new_size.take() {
                    viewer.handle_screen_resize(size);
                }

                let pixels_per_point = window.window().scale_factor() as f32;

                egui_winit_state.set_pixels_per_point(pixels_per_point);
                let egui_input =
                    egui_winit_state.take_egui_input(window.window());

                let gui_state = GuiState {
                    status: &status,
                    model_available: host.is_some(),
                };
                let new_model_path =
                    viewer.draw(pixels_per_point, egui_input, gui_state);

                if let Some(model_path) = new_model_path {
                    let model =
                        Model::new(model_path, Parameters::empty()).unwrap();
                    match Host::from_model(model) {
                        Ok(new_host) => {
                            host = Some(new_host);
                        }
                        Err(_) => {
                            status.update_status("Error creating host.");
                        }
                    }
                }
            }
            _ => {}
        }

        let input_event = input_event(
            &event,
            &window,
            &held_mouse_button,
            &mut viewer.cursor,
            invert_zoom,
        );
        if let Some(input_event) = input_event {
            viewer.handle_input_event(input_event);
        }
    });
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

/// Error in main loop
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error loading model
    #[error("Error loading model")]
    Model(#[from] fj_host::Error),

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
