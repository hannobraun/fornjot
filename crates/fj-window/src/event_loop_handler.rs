use fj_host::{Host, Model, ModelEvent, Parameters};
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{
    GuiState, InputEvent, NormalizedScreenPosition, Screen, ScreenSize,
    StatusReport, Viewer,
};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
    event_loop::ControlFlow,
};

use crate::window::Window;

pub struct EventLoopHandler {
    pub invert_zoom: bool,
    pub shape_processor: ShapeProcessor,
    pub window: Window,
    pub viewer: Viewer,
    pub egui_winit_state: egui_winit::State,
    pub host: Option<Host>,
    pub status: StatusReport,
    pub held_mouse_button: Option<MouseButton>,

    /// Only handle resize events once every frame. This filters out spurious
    /// resize events that can lead to wgpu warnings. See this issue for some
    /// context:
    /// <https://github.com/rust-windowing/winit/issues/2094>
    pub new_size: Option<ScreenSize>,
}

impl EventLoopHandler {
    #[allow(clippy::result_large_err)]
    pub fn handle_event(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) -> Result<(), fj_operations::shape_processor::Error> {
        if let Some(host) = &self.host {
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
                        self.status.update_status(
                            "Change in model detected. Evaluating model...",
                        );
                    }
                    ModelEvent::Evaluation(evaluation) => {
                        self.status.update_status(
                            "Model evaluated. Processing model...",
                        );

                        let shape =
                            self.shape_processor.process(&evaluation.shape)?;
                        self.viewer.handle_shape_update(shape);

                        self.status.update_status("Model processed.");
                    }

                    ModelEvent::Error(err) => {
                        self.status.update_status(&err.to_string());
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
            self.egui_winit_state
                .on_event(self.viewer.gui.context(), event);
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
                    self.viewer.toggle_draw_model();
                }
                VirtualKeyCode::Key2 => {
                    self.viewer.toggle_draw_mesh();
                }
                VirtualKeyCode::Key3 => {
                    self.viewer.toggle_draw_debug();
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                self.new_size = Some(ScreenSize {
                    width: size.width,
                    height: size.height,
                });
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => match state {
                ElementState::Pressed => {
                    self.held_mouse_button = Some(button);
                    self.viewer.add_focus_point();
                }
                ElementState::Released => {
                    self.held_mouse_button = None;
                    self.viewer.remove_focus_point();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { .. },
                ..
            } => self.viewer.add_focus_point(),
            Event::MainEventsCleared => {
                self.window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Only do a screen resize once per frame. This protects against
                // spurious resize events that cause issues with the renderer.
                if let Some(size) = self.new_size.take() {
                    self.viewer.handle_screen_resize(size);
                }

                let pixels_per_point =
                    self.window.window().scale_factor() as f32;

                self.egui_winit_state.set_pixels_per_point(pixels_per_point);
                let egui_input =
                    self.egui_winit_state.take_egui_input(self.window.window());

                let gui_state = GuiState {
                    status: &self.status,
                    model_available: self.host.is_some(),
                };
                let new_model_path =
                    self.viewer.draw(pixels_per_point, egui_input, gui_state);

                if let Some(model_path) = new_model_path {
                    let model =
                        Model::new(model_path, Parameters::empty()).unwrap();
                    match Host::from_model(model) {
                        Ok(new_host) => {
                            self.host = Some(new_host);
                        }
                        Err(err) => {
                            self.status.update_status(&format!(
                                "Error creating host: {err}"
                            ));
                        }
                    }
                }
            }
            _ => {}
        }

        let input_event = input_event(
            &event,
            &self.window,
            &self.held_mouse_button,
            &mut self.viewer.cursor,
            self.invert_zoom,
        );
        if let Some(input_event) = input_event {
            self.viewer.handle_input_event(input_event);
        }

        Ok(())
    }
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
