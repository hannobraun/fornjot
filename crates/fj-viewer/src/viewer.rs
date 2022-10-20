use fj_interop::{
    processed_shape::ProcessedShape, status_report::StatusReport,
};
use fj_math::Aabb;
use tracing::warn;

use crate::{
    camera::FocusPoint, gui::Gui, Camera, DrawConfig, InputEvent, InputHandler,
    NormalizedScreenPosition, Renderer, RendererInitError, Screen, ScreenSize,
};

/// The Fornjot model viewer
pub struct Viewer {
    /// The camera
    pub camera: Camera,

    /// The cursor
    pub cursor: Option<NormalizedScreenPosition>,

    /// The draw config
    pub draw_config: DrawConfig,

    /// The focus point
    pub focus_point: Option<FocusPoint>,

    /// The GUI
    pub gui: Gui,

    /// The input handler
    pub input_handler: InputHandler,

    /// The renderer
    pub renderer: Renderer,

    /// The shape
    pub shape: Option<ProcessedShape>,
}

impl Viewer {
    /// Construct a new instance of `Viewer`
    pub async fn new(screen: &impl Screen) -> Result<Self, RendererInitError> {
        let renderer = Renderer::new(screen).await?;
        let gui = renderer.init_gui();

        Ok(Self {
            camera: Camera::default(),
            cursor: None,
            draw_config: DrawConfig::default(),
            focus_point: None,
            gui,
            input_handler: InputHandler::default(),
            renderer,
            shape: None,
        })
    }

    /// Toggle the "draw model" setting
    pub fn toggle_draw_model(&mut self) {
        self.draw_config.draw_model = !self.draw_config.draw_model
    }

    /// Toggle the "draw mesh" setting
    pub fn toggle_draw_mesh(&mut self) {
        if self.renderer.is_line_drawing_available() {
            self.draw_config.draw_mesh = !self.draw_config.draw_mesh
        }
    }

    /// Toggle the "draw debug" setting
    pub fn toggle_draw_debug(&mut self) {
        if self.renderer.is_line_drawing_available() {
            self.draw_config.draw_debug = !self.draw_config.draw_debug
        }
    }

    /// Handle the shape being updated
    pub fn handle_shape_update(&mut self, shape: ProcessedShape) {
        self.renderer
            .update_geometry((&shape.mesh).into(), (&shape.debug_info).into());

        let aabb = shape.aabb;
        if self.shape.replace(shape).is_none() {
            self.camera.init_planes(&aabb)
        }
    }

    /// Handle an input event
    pub fn handle_input_event(&mut self, event: InputEvent) {
        if let Some(focus_point) = self.focus_point {
            self.input_handler.handle_event(
                event,
                focus_point,
                &mut self.camera,
            );
        }
    }

    /// Handle the screen being resized
    pub fn handle_screen_resize(&mut self, screen_size: ScreenSize) {
        self.renderer.handle_resize(screen_size)
    }

    /// Compute and store a focus point, unless one is already stored
    pub fn add_focus_point(&mut self) {
        // Don't recompute the focus point unnecessarily.
        if let Some(shape) = &self.shape {
            if self.focus_point.is_none() {
                self.focus_point =
                    Some(self.camera.focus_point(self.cursor, shape));
            }
        }
    }

    /// Remove the stored focus point
    pub fn remove_focus_point(&mut self) {
        self.focus_point = None;
    }

    /// Draw the graphics
    pub fn draw(
        &mut self,
        scale_factor: f32,
        status: &mut StatusReport,
        egui_input: egui::RawInput,
    ) {
        let aabb = self
            .shape
            .as_ref()
            .map(|shape| shape.aabb)
            .unwrap_or_else(Aabb::default);

        self.camera.update_planes(&aabb);

        self.gui.update(
            egui_input,
            &mut self.draw_config,
            &aabb,
            status,
            self.renderer.is_line_drawing_available(),
        );

        if let Err(err) = self.renderer.draw(
            &self.camera,
            &self.draw_config,
            scale_factor,
            &mut self.gui,
        ) {
            warn!("Draw error: {}", err);
        }
    }
}
