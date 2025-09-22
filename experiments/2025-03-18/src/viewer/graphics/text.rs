pub struct TextRenderer {}

impl TextRenderer {
    pub fn new(_: &wgpu::Device) -> Self {
        Self {}
    }

    pub fn draw(&mut self, _: &mut wgpu::RenderPass) {}
}
