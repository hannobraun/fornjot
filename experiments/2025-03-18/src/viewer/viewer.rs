use std::{
    collections::BTreeMap,
    panic,
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

use fj_interop::TriMesh;
use fj_math::Point;
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
    approx::face::FaceApproxPoints,
    viewer::{
        RendererInitError,
        input::DEFAULT_CAMERA_TUNING_CONFIG,
        window::{Displayable, PointWithLabel, Window},
    },
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
    f: impl FnOnce(ViewerHandle) -> R + Send + 'static,
) -> Result<R, Error>
where
    R: Send + 'static,
{
    let event_loop = EventLoop::with_user_event().build()?;

    let mut display_state = Viewer {
        windows: BTreeMap::new(),
        id_map: BTreeMap::new(),
    };

    let proxy = event_loop.create_proxy();
    let handle = thread::spawn(|| {
        f(ViewerHandle {
            next_window_id: AtomicU64::new(0),
            event_loop: EventLoopProxy { inner: proxy },
        })
    });

    event_loop.run_app(&mut display_state)?;

    let result = match handle.join() {
        Ok(result) => result,
        Err(payload) => panic::resume_unwind(payload),
    };

    Ok(result)
}

/// # Handle to the model viewer
pub struct ViewerHandle {
    next_window_id: AtomicU64,
    event_loop: EventLoopProxy,
}

impl ViewerHandle {
    /// # Open a new window
    ///
    /// The new window will not be shown, initially. You must display something
    /// first, by calling one of the [`WindowHandle`]'s methods.
    pub fn open_window(&self) -> WindowHandle {
        // Use a conservative ordering, just to be on the safe side. This code
        // shouldn't be performance-sensitive anyway.
        let id = self.next_window_id.fetch_add(1, Ordering::SeqCst);

        self.event_loop.send_event(EventLoopEvent::Window { id });

        WindowHandle {
            id,
            event_loop: self.event_loop.clone(),
        }
    }
}

/// # Handle to a model viewer window
///
/// See [`ViewerHandle::open_window`].
pub struct WindowHandle {
    id: u64,
    event_loop: EventLoopProxy,
}

impl WindowHandle {
    /// # Display a face in surface space
    pub fn display_face_surface(&self, face: &FaceApproxPoints) -> &Self {
        let points = face
            .points
            .iter()
            .copied()
            .map(|point| PointWithLabel {
                point: point.point_surface.to_xyz(),
                label: point,
            })
            .collect::<Vec<_>>();

        self.event_loop.send_event(EventLoopEvent::Displayable {
            displayable: Displayable::face(points),
            window_id: self.id,
        });

        self
    }

    /// # Display a face in global space
    pub fn display_face_global(&self, face: &FaceApproxPoints) -> &Self {
        let points = face
            .points
            .iter()
            .copied()
            .map(|point| PointWithLabel {
                point: point.point_global,
                label: point,
            })
            .collect();

        self.event_loop.send_event(EventLoopEvent::Displayable {
            displayable: Displayable::face(points),
            window_id: self.id,
        });

        self
    }

    /// # Display a 3D triangle mesh
    pub fn display_mesh(&self, tri_mesh: TriMesh) -> &Self {
        self.event_loop.send_event(EventLoopEvent::Displayable {
            displayable: Displayable::mesh(tri_mesh),
            window_id: self.id,
        });

        self
    }

    /// # Display a 2D point
    ///
    /// Please note that currently the point is only displayed as a single
    /// pixel. Depending on your resolution, that might mean that it's barely
    /// visible.
    pub fn display_point_surface(&self, point: impl Into<Point<2>>) -> &Self {
        self.event_loop.send_event(EventLoopEvent::Displayable {
            displayable: Displayable::Point {
                point: point.into().to_xyz(),
            },
            window_id: self.id,
        });

        self
    }

    /// # Display a 3D point
    ///
    /// Please note that currently the point is only displayed as a single
    /// pixel. Depending on your resolution, that might mean that it's barely
    /// visible.
    pub fn display_point_global(&self, point: impl Into<Point<3>>) -> &Self {
        self.event_loop.send_event(EventLoopEvent::Displayable {
            displayable: Displayable::Point {
                point: point.into(),
            },
            window_id: self.id,
        });

        self
    }

    /// # Clear the contents of the window
    pub fn clear(&self) -> &Self {
        self.event_loop
            .send_event(EventLoopEvent::Clear { window_id: self.id });

        self
    }
}

#[derive(Clone)]
struct EventLoopProxy {
    inner: winit::event_loop::EventLoopProxy<EventLoopEvent>,
}

impl EventLoopProxy {
    fn send_event(&self, event: EventLoopEvent) {
        // If there's an error, that means the display thread has closed down
        // and we're on our way to shutting down as well. I don't think there's
        // much we can do about that.
        let _ = self.inner.send_event(event);
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

struct Viewer {
    windows: BTreeMap<WindowId, Window>,
    id_map: BTreeMap<u64, WindowId>,
}

impl ApplicationHandler<EventLoopEvent> for Viewer {
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
                    window.toggle_draw_mesh_triangles();
                }
                Key::Character("2") => {
                    window.toggle_draw_mesh_lines();
                }
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                window.on_cursor_movement([position.x, position.y]);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    MouseButton::Left => {
                        Some(crate::viewer::input::MouseButton::Left)
                    }
                    MouseButton::Right => {
                        Some(crate::viewer::input::MouseButton::Right)
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
                drawn = window.draw();
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
        match event {
            EventLoopEvent::Window { id: window_id } => {
                let window = block_on(Window::new(event_loop)).unwrap();
                let winit_window_id = window.winit_window().id();

                self.windows.insert(winit_window_id, window);
                self.id_map.insert(window_id, winit_window_id);
            }
            EventLoopEvent::Displayable {
                displayable,
                window_id,
            } => {
                let Some(winit_window_id) = self.id_map.get(&window_id) else {
                    unreachable!(
                        "Mappings for all window IDs are created when handling \
                        the `Window` event."
                    );
                };
                let Some(window) = self.windows.get_mut(winit_window_id) else {
                    unreachable!(
                        "We never remove any windows, so it's not possible to \
                        have a mapping to an ID, but not a window with that ID."
                    );
                };

                window.add_displayable(displayable);
            }
            EventLoopEvent::Clear { window_id } => {
                let Some(winit_window_id) = self.id_map.get(&window_id) else {
                    unreachable!(
                        "Mappings for all window IDs are created when handling \
                        the `Window` event."
                    );
                };
                let Some(window) = self.windows.get_mut(winit_window_id) else {
                    unreachable!(
                        "We never remove any windows, so it's not possible to \
                        have a mapping to an ID, but not a window with that ID."
                    );
                };

                window.clear();
            }
        }
    }
}

enum EventLoopEvent {
    Window {
        id: u64,
    },
    Displayable {
        displayable: Displayable,
        window_id: u64,
    },
    Clear {
        window_id: u64,
    },
}
