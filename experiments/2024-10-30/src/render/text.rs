pub struct TextRenderer {
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let cache = glyphon::Cache::new(device);

        let mut text_atlas = glyphon::TextAtlas::new(
            device,
            queue,
            &cache,
            surface_config.format,
        );

        let viewport = glyphon::Viewport::new(device, &cache);

        let text_renderer = glyphon::TextRenderer::new(
            &mut text_atlas,
            device,
            wgpu::MultisampleState::default(),
            None,
        );

        Self {
            text_atlas,
            viewport,
            text_renderer,
        }
    }

    pub fn render(
        &self,
        render_pass: &mut wgpu::RenderPass,
    ) -> anyhow::Result<()> {
        self.text_renderer.render(
            &self.text_atlas,
            &self.viewport,
            render_pass,
        )?;

        Ok(())
    }
}
