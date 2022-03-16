#[derive(Debug)]
pub struct DrawConfig {
    pub draw_model: bool,
    pub draw_mesh: bool,
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
