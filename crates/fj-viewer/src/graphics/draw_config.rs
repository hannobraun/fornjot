/// High level configuration for rendering the active model
#[derive(Debug)]
pub struct DrawConfig {
    /// Toggle for displaying the shaded model
    pub draw_model: bool,

    /// Toggle for displaying the wireframe model
    pub draw_mesh: bool,

    /// Toggle for displaying model debug information
    pub draw_debug: bool,
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self {
            draw_model: true,
            draw_mesh: false,
            draw_debug: false,
        }
    }
}
