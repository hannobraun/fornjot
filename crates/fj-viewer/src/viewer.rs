use fj_interop::processed_shape::ProcessedShape;

use crate::{
    Camera, DrawConfig, InputHandler, Renderer, RendererInitError, Screen,
};

/// The Fornjot model viewer
pub struct Viewer {
    /// The camera
    pub camera: Camera,

    /// The draw config
    pub draw_config: DrawConfig,

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
        Ok(Self {
            camera: Camera::default(),
            draw_config: DrawConfig::default(),
            input_handler: InputHandler::default(),
            renderer: Renderer::new(screen).await?,
            shape: None,
        })
    }

    /// Handle the shape being updated
    pub fn handle_shape_update(&mut self, shape: ProcessedShape) {
        self.renderer.update_geometry(
            (&shape.mesh).into(),
            (&shape.debug_info).into(),
            shape.aabb,
        );
        self.camera.update_planes(&shape.aabb);

        self.shape = Some(shape);
    }
}
