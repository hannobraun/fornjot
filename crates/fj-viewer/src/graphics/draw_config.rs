#[derive(Debug)]
pub struct DrawConfig {
    pub draw_mesh_triangles: bool,
    pub draw_mesh_lines: bool,
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self {
            draw_mesh_triangles: true,
            draw_mesh_lines: false,
        }
    }
}
