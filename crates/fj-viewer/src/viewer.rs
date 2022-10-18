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
}

impl Viewer {
    /// Construct a new instance of `Viewer`
    pub async fn new(screen: &impl Screen) -> Result<Self, RendererInitError> {
        Ok(Self {
            camera: Camera::default(),
            draw_config: DrawConfig::default(),
            input_handler: InputHandler::default(),
            renderer: Renderer::new(screen).await?,
        })
    }
}
