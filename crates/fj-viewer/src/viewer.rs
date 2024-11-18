use fj_interop::Model;
use tracing::warn;

use crate::{
    camera::{Camera, FocusPoint},
    graphics::{DrawConfig, Renderer},
    CameraTuningConfig, InputEvent, MouseButton, NormalizedScreenPosition,
    RendererInitError, Screen, ScreenSize, DEFAULT_CAMERA_TUNING_CONFIG,
};

/// The Fornjot model viewer
pub struct Viewer {
    current_screen_size: ScreenSize,
    new_screen_size: Option<ScreenSize>,
    most_recent_mouse_button: Option<MouseButton>,
    camera_tuning_config: CameraTuningConfig,
    camera: Camera,
    cursor: Option<NormalizedScreenPosition>,
    draw_config: DrawConfig,
    focus_point: Option<FocusPoint>,
    renderer: Renderer,
    model: Option<Model>,
}

impl Viewer {
    /// Construct a new instance of `Viewer`
    pub async fn new(screen: &impl Screen) -> Result<Self, RendererInitError> {
        let renderer = Renderer::new(screen).await?;

        Ok(Self {
            current_screen_size: screen.size(),
            new_screen_size: None,
            most_recent_mouse_button: None,
            camera_tuning_config: DEFAULT_CAMERA_TUNING_CONFIG,
            camera: Camera::default(),
            cursor: None,
            draw_config: DrawConfig::default(),
            focus_point: None,
            renderer,
            model: None,
        })
    }

    /// Access the cursor
    pub fn cursor(&mut self) -> &mut Option<NormalizedScreenPosition> {
        &mut self.cursor
    }

    /// Toggle the "draw model" setting
    pub fn toggle_draw_model(&mut self) {
        self.draw_config.draw_model = !self.draw_config.draw_model;
    }

    /// Toggle the "draw mesh" setting
    pub fn toggle_draw_mesh(&mut self) {
        self.draw_config.draw_mesh = !self.draw_config.draw_mesh;
    }

    /// Handle the model being updated
    pub fn handle_model_update(&mut self, model: Model) {
        self.renderer.update_geometry((&model.mesh).into());

        let aabb = model.aabb;
        if self.model.replace(model).is_none() {
            self.camera.init_planes(&aabb);
        }
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
        let [width, height] = self.current_screen_size.as_f64();
        let aspect_ratio = width / height;

        // Cursor position in normalized coordinates (-1 to +1) with aspect
        // ratio taken into account.
        let cursor_new = NormalizedScreenPosition {
            x: x / width * 2. - 1.,
            y: -(y / height * 2. - 1.) / aspect_ratio,
        };

        let event = if let (Some(cursor_old), Some(button)) =
            (self.cursor, self.most_recent_mouse_button)
        {
            match button {
                MouseButton::Left => {
                    let diff_x = cursor_new.x - cursor_old.x;
                    let diff_y = cursor_new.y - cursor_old.y;
                    let angle_x = -diff_y
                        * self.camera_tuning_config.rotation_sensitivity;
                    let angle_y =
                        diff_x * self.camera_tuning_config.rotation_sensitivity;

                    if let Some(focus_point) = self.focus_point {
                        self.camera.apply_rotation(
                            angle_x,
                            angle_y,
                            focus_point,
                        );
                    }

                    None
                }
                MouseButton::Right => {
                    if let Some(focus_point) = self.focus_point {
                        self.camera.apply_translation(
                            cursor_old,
                            cursor_new,
                            focus_point,
                        );
                    }
                    None
                }
            }
        } else {
            None
        };
        if let Some(event) = event {
            self.handle_input_event(event);
        }

        self.cursor = Some(cursor_new);
    }

    /// # Handle a mouse button being pressed
    pub fn on_mouse_button_pressed(&mut self, button: MouseButton) {
        self.most_recent_mouse_button = Some(button);
    }

    /// # Handle a mouse button being pressed
    pub fn on_mouse_button_released(&mut self, button: MouseButton) {
        if self.most_recent_mouse_button == Some(button) {
            self.most_recent_mouse_button = None;
        }
    }

    /// Handle the screen being resized
    pub fn on_screen_resize(&mut self, new_size: ScreenSize) {
        self.current_screen_size = new_size;
        self.new_screen_size = Some(new_size);
    }

    /// Compute and store a focus point, unless one is already stored
    pub fn add_focus_point(&mut self) {
        if let Some(model) = &self.model {
            if self.focus_point.is_none() {
                self.focus_point =
                    Some(self.camera.focus_point(self.cursor, model));
            }
        }
    }

    /// Remove the stored focus point
    pub fn remove_focus_point(&mut self) {
        self.focus_point = None;
    }

    /// Draw the graphics
    pub fn draw(&mut self) {
        if !self.current_screen_size.is_valid() {
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
            .map(|shape| shape.aabb)
            .unwrap_or_default();

        self.camera.update_planes(&aabb);

        if let Err(err) = self.renderer.draw(&self.camera, &self.draw_config) {
            warn!("Draw error: {}", err);
        }
    }
}
