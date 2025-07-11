/// High level configuration for rendering the active model
#[derive(Debug)]
pub struct DrawConfig {
    /// Toggle for displaying the shaded model
    pub draw_mesh_triangles: bool,

    /// Toggle for displaying the wireframe model
    pub draw_mesh: bool,
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self {
            draw_mesh_triangles: true,
            draw_mesh: false,
        }
    }
}
