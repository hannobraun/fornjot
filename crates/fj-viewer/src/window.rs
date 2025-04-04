use std::sync::Arc;

use fj_interop::TriMesh;
use fj_math::Aabb;
use tracing::warn;
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop};

use crate::{
    RendererInitError,
    camera::{Camera, FocusPoint},
    graphics::{DrawConfig, Renderer, Vertices},
    input::{
        CameraTuningConfig, DEFAULT_CAMERA_TUNING_CONFIG, InputEvent,
        MouseButton,
    },
};

pub struct Window {
    new_screen_size: Option<PhysicalSize<u32>>,
    most_recent_mouse_button: Option<MouseButton>,
    camera_tuning_config: CameraTuningConfig,
    camera: Camera,
    cursor: Option<NormalizedScreenPosition>,
    draw_config: DrawConfig,
    focus_point: Option<FocusPoint>,
    window: Arc<winit::window::Window>,
    renderer: Renderer,
    model: Option<(TriMesh, Aabb<3>)>,
}

impl Window {
    pub async fn new(
        tri_mesh: TriMesh,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, WindowError> {
        let aabb = tri_mesh.aabb();

        let window = Arc::new(
            event_loop.create_window(
                winit::window::Window::default_attributes()
                    .with_title("Fornjot")
                    .with_decorations(true)
                    .with_transparent(false),
            )?,
        );
        let renderer =
            Renderer::new(window.clone(), Vertices::from_tri_mesh(&tri_mesh))
                .await?;
        let camera = Camera::new(&aabb);

        Ok(Self {
            new_screen_size: None,
            most_recent_mouse_button: None,
            camera_tuning_config: DEFAULT_CAMERA_TUNING_CONFIG,
            camera,
            cursor: None,
            draw_config: DrawConfig::default(),
            focus_point: None,
            window,
            renderer,
            model: Some((tri_mesh, aabb)),
        })
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        &self.window
    }

    /// Toggle the "draw model" setting
    pub fn toggle_draw_model(&mut self) {
        self.draw_config.draw_model = !self.draw_config.draw_model;
    }

    /// Toggle the "draw mesh" setting
    pub fn toggle_draw_mesh(&mut self) {
        self.draw_config.draw_mesh = !self.draw_config.draw_mesh;
    }

    /// Handle an input event
    pub fn handle_input_event(&mut self, event: InputEvent) {
        let Some(focus_point) = self.focus_point else {
            return;
        };

        match event {
            InputEvent::Zoom(zoom_delta) => {
                self.camera.apply_zoom(zoom_delta, focus_point);
            }
        };
    }

    /// # Handle a cursor movement
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
                        * self.camera_tuning_config.rotation_sensitivity;
                    let angle_y =
                        diff_x * self.camera_tuning_config.rotation_sensitivity;

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

        self.remove_focus_point();
    }

    /// Handle the screen being resized
    pub fn on_screen_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.new_screen_size = Some(new_size);
    }

    /// Compute and store a focus point, unless one is already stored
    pub fn add_focus_point(&mut self) {
        if let Some((mesh, aabb)) = &self.model {
            if self.focus_point.is_none() {
                self.focus_point =
                    Some(self.camera.focus_point(self.cursor, mesh, aabb));
            }
        }
    }

    /// Remove the stored focus point
    pub fn remove_focus_point(&mut self) {
        self.focus_point = None;
    }

    /// Draw the graphics
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

        let aabb = self
            .model
            .as_ref()
            .map(|(_, aabb)| *aabb)
            .unwrap_or_default();

        self.camera.update_planes(&aabb);

        if let Err(err) = self.renderer.draw(&self.camera, &self.draw_config) {
            warn!("Draw error: {}", err);
        }
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
