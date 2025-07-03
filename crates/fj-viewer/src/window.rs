use std::sync::Arc;

use fj_interop::TriMesh;
use fj_math::{Aabb, Point};
use tracing::warn;
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop};

use crate::{
    RendererInitError,
    camera::{Camera, FocusPoint},
    graphics::{DrawConfig, RenderMode, Renderer, Vertices},
    input::{DEFAULT_CAMERA_TUNING_CONFIG, MouseButton},
};

pub struct Window {
    new_screen_size: Option<PhysicalSize<u32>>,
    most_recent_mouse_button: Option<MouseButton>,
    camera: Camera,
    cursor: Option<NormalizedScreenPosition>,
    draw_config: DrawConfig,
    focus_point: Option<FocusPoint>,
    window: Arc<winit::window::Window>,
    renderer: Renderer,
    tri_mesh: TriMesh,
    aabb: Aabb<3>,
}

impl Window {
    pub async fn new(
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, WindowError> {
        let tri_mesh = TriMesh::default();
        let aabb = Aabb::<3>::default();

        let window = Arc::new(
            event_loop.create_window(
                winit::window::Window::default_attributes()
                    .with_title("Fornjot")
                    .with_decorations(true)
                    .with_transparent(false),
            )?,
        );
        let renderer = Renderer::new(window.clone()).await?;

        let camera = Camera::new(&aabb);

        Ok(Self {
            new_screen_size: None,
            most_recent_mouse_button: None,
            camera,
            cursor: None,
            draw_config: DrawConfig::default(),
            focus_point: None,
            window,
            renderer,
            tri_mesh,
            aabb,
        })
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        &self.window
    }

    /// # Toggle the "draw model" setting
    pub fn toggle_draw_model(&mut self) {
        self.draw_config.draw_model = !self.draw_config.draw_model;
    }

    /// # Toggle the "draw mesh" setting
    pub fn toggle_draw_mesh(&mut self) {
        self.draw_config.draw_mesh = !self.draw_config.draw_mesh;
    }

    /// # Compute and store a focus point, unless one is already stored
    pub fn add_focus_point(&mut self) {
        if self.focus_point.is_none() {
            self.focus_point = Some(self.camera.focus_point(
                self.cursor,
                &self.tri_mesh,
                &self.aabb,
            ));
        }
    }

    /// # Handle the screen being resized
    pub fn on_screen_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.new_screen_size = Some(new_size);
    }

    /// # Handle cursor movement
    pub fn on_cursor_movement(&mut self, [x, y]: [f64; 2]) {
        let [width, height]: [f64; 2] = {
            let size = self.window.inner_size();
            [size.width, size.height].map(Into::into)
        };
        let aspect_ratio = width / height;

        // Cursor position in normalized coordinates (-1 to +1) with aspect
        // ratio taken into account.
        let cursor_new = NormalizedScreenPosition {
            x: x / width * 2. - 1.,
            y: -(y / height * 2. - 1.) / aspect_ratio,
        };

        if let (Some(cursor_old), Some(button), Some(focus_point)) =
            (self.cursor, self.most_recent_mouse_button, self.focus_point)
        {
            match button {
                MouseButton::Left => {
                    let diff_x = cursor_new.x - cursor_old.x;
                    let diff_y = cursor_new.y - cursor_old.y;
                    let angle_x = -diff_y
                        * DEFAULT_CAMERA_TUNING_CONFIG.rotation_sensitivity;
                    let angle_y = diff_x
                        * DEFAULT_CAMERA_TUNING_CONFIG.rotation_sensitivity;

                    self.camera.apply_rotation(angle_x, angle_y, focus_point);
                }
                MouseButton::Right => {
                    self.camera.apply_translation(
                        cursor_old,
                        cursor_new,
                        focus_point,
                    );
                }
            }
        }

        self.cursor = Some(cursor_new);
    }

    /// # Handle a mouse button being pressed
    pub fn on_mouse_button_pressed(&mut self, button: MouseButton) {
        self.most_recent_mouse_button = Some(button);
        self.add_focus_point();
    }

    /// # Handle a mouse button being pressed
    pub fn on_mouse_button_released(&mut self, button: MouseButton) {
        if self.most_recent_mouse_button == Some(button) {
            self.most_recent_mouse_button = None;
        }

        self.focus_point = None;
    }

    /// # Handle zoom
    pub fn on_zoom(&mut self, delta: f64) {
        let Some(focus_point) = self.focus_point else {
            return;
        };

        self.camera.apply_zoom(delta, focus_point);
    }

    pub fn add_displayable(&mut self, displayable: Displayable) {
        let (render_mode, vertices, aabb) = match displayable {
            Displayable::Face { points, aabb } => {
                let vertices = Vertices::for_face(&points);
                let render_mode = RenderMode::Face;

                (render_mode, vertices, aabb)
            }
            Displayable::Model { tri_mesh, aabb } => {
                let vertices = Vertices::for_model(&tri_mesh);
                let render_mode = RenderMode::Model;

                self.tri_mesh = self.tri_mesh.clone().merge(tri_mesh);

                (render_mode, vertices, aabb)
            }
        };

        self.renderer.add_geometry(render_mode, vertices);

        self.aabb = self.aabb.merged(&aabb);
        self.camera = Camera::new(&self.aabb);
    }

    /// # Draw the window
    pub fn draw(&mut self) {
        let size_is_invalid = {
            let size = self.window.inner_size();
            size.width == 0 || size.height == 0
        };
        if size_is_invalid {
            return;
        }

        if let Some(new_size) = self.new_screen_size.take() {
            // We should only supply valid screen sizes to the renderer. But
            // `self.current_screen_size` has already been updated, and we're
            // checking if that's valid above. No need to check again.
            self.renderer.handle_resize(new_size);
        }

        self.camera.update_planes(&self.aabb);

        if let Err(err) = self.renderer.draw(&self.camera, &self.draw_config) {
            warn!("Draw error: {}", err);
        }
    }
}

pub enum Displayable {
    Face {
        points: Vec<Point<2>>,
        aabb: Aabb<3>,
    },
    Model {
        tri_mesh: TriMesh,
        aabb: Aabb<3>,
    },
}

impl Displayable {
    pub fn face(points: Vec<Point<2>>) -> Self {
        let aabb =
            Aabb::<3>::from_points(points.iter().map(|point| point.to_xyz()));
        Self::Face { points, aabb }
    }

    pub fn model(tri_mesh: TriMesh) -> Self {
        let aabb = tri_mesh.aabb();
        Self::Model { tri_mesh, aabb }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Failed to initialize window")]
    WindowInit(#[from] winit::error::OsError),

    #[error(transparent)]
    RendererInit(#[from] RendererInitError),
}

/// Cursor position in normalized coordinates (-1 to +1)
///
/// The center of the screen is at (0, 0). The aspect ratio is taken into
/// account.
#[derive(Clone, Copy, Debug)]
pub struct NormalizedScreenPosition {
    /// The x coordinate of the position [-1, 1]
    pub x: f64,

    /// The y coordinate of the position [-1, 1]
    pub y: f64,
}
