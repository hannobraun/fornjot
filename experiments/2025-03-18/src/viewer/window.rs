use std::sync::Arc;

use fj_interop::TriMesh;
use fj_math::{Aabb, Point};
use tracing::warn;
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop};

use crate::{
    approx::point::ApproxPoint,
    viewer::{
        RendererInitError,
        camera::{Camera, FocusPoint},
        graphics::{DrawConfig, RenderMode, Renderer, Vertices},
        input::{DEFAULT_CAMERA_TUNING_CONFIG, MouseButton},
    },
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
    should_render: bool,
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
            should_render: false,
        })
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        &self.window
    }

    /// # Toggle whether the triangles of meshes are drawn
    pub fn toggle_draw_mesh_triangles(&mut self) {
        self.draw_config.draw_mesh_triangles =
            !self.draw_config.draw_mesh_triangles;
    }

    /// # Toggle whether the lines of meshes are drawn
    pub fn toggle_draw_mesh_lines(&mut self) {
        self.draw_config.draw_mesh_lines = !self.draw_config.draw_mesh_lines;
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
        let (render_mode, vertices, labels, aabb) = match displayable {
            Displayable::Face { points } => {
                let render_mode = RenderMode::Face;
                let vertices = Vertices::for_face(
                    points.iter().map(|PointWithLabel { point, .. }| point),
                );
                let labels = points
                    .iter()
                    .map(|PointWithLabel { point, label }| {
                        (
                            format!(
                                "{point_surface:.3?} / {point_global:.3?}",
                                point_surface = label.point_surface,
                                point_global = label.point_global,
                            ),
                            *point,
                        )
                    })
                    .collect();
                let aabb = Aabb::<3>::from_points(
                    points
                        .iter()
                        .map(|PointWithLabel { point, .. }| point)
                        .copied(),
                );

                (render_mode, vertices, labels, aabb)
            }
            Displayable::Mesh { tri_mesh } => {
                let render_mode = RenderMode::Mesh;
                let vertices = Vertices::for_mesh(&tri_mesh);
                let labels = vec![];
                let aabb = tri_mesh.aabb();

                self.tri_mesh = self.tri_mesh.clone().merge(tri_mesh);

                (render_mode, vertices, labels, aabb)
            }
            Displayable::Point { point } => {
                let render_mode = RenderMode::Point;
                let vertices = Vertices::for_point(point);
                let labels = vec![];

                let aabb = Aabb {
                    min: point,
                    max: point,
                };

                (render_mode, vertices, labels, aabb)
            }
        };

        self.renderer.add_geometry(render_mode, vertices, labels);

        self.aabb = self.aabb.merged(&aabb);
        self.camera = Camera::new(&self.aabb);

        self.should_render = true;
    }

    /// # Clear the geometry displayed in the window
    pub fn clear(&mut self) {
        self.renderer.clear_geometry();
    }

    /// # Draw the window
    pub fn draw(&mut self) -> bool {
        let size_is_invalid = {
            let size = self.window.inner_size();
            size.width == 0 || size.height == 0
        };
        if size_is_invalid {
            return false;
        }

        if let Some(new_size) = self.new_screen_size.take() {
            // We should only supply valid screen sizes to the renderer. But
            // `self.current_screen_size` has already been updated, and we're
            // checking if that's valid above. No need to check again.
            self.renderer.handle_resize(new_size);
        }

        self.camera.update_planes(&self.aabb);

        if self.should_render {
            if let Err(err) =
                self.renderer.draw(&self.camera, &self.draw_config)
            {
                warn!("Draw error: {}", err);
            }

            true
        } else {
            false
        }
    }
}

pub enum Displayable {
    Face { points: Vec<PointWithLabel> },
    Mesh { tri_mesh: TriMesh },
    Point { point: Point<3> },
}

impl Displayable {
    pub fn face(points: Vec<PointWithLabel>) -> Self {
        Self::Face { points }
    }

    pub fn mesh(tri_mesh: TriMesh) -> Self {
        Self::Mesh { tri_mesh }
    }
}

pub struct PointWithLabel {
    pub point: Point<3>,
    pub label: ApproxPoint,
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
