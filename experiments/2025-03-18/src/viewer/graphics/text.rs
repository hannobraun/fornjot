pub struct TextRenderer {
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let cache = glyphon::Cache::new(device);

        let mut text_atlas =
            glyphon::TextAtlas::new(device, queue, &cache, color_format);
        let viewport = glyphon::Viewport::new(device, &cache);

        let multisample_state = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };
        let depth_stencil = None;
        let text_renderer = glyphon::TextRenderer::new(
            &mut text_atlas,
            device,
            multisample_state,
            depth_stencil,
        );

        Self {
            text_atlas,
            viewport,
            text_renderer,
        }
    }

    pub fn draw(
        &mut self,
        _: &wgpu::Device,
        _: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
    ) -> Result<(), glyphon::RenderError> {
        self.text_renderer.render(
            &self.text_atlas,
            &self.viewport,
            render_pass,
        )?;

        Ok(())
    }
}
