pub struct TextRenderer {}

impl TextRenderer {
    pub fn new(
        _: &wgpu::Device,
        _: &wgpu::Queue,
        _: wgpu::TextureFormat,
    ) -> Self {
        Self {}
    }

    pub fn draw(
        &mut self,
        _: &mut wgpu::RenderPass,
    ) -> Result<(), glyphon::RenderError> {
        Ok(())
    }
}
