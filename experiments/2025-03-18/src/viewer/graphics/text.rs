use glyphon::FontSystem;

pub struct TextRenderer {
    text_atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    font_system: glyphon::FontSystem,
    viewport: glyphon::Viewport,
    swash_cache: glyphon::SwashCache,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let cache = glyphon::Cache::new(device);

        let font_system = FontSystem::new();
        let mut text_atlas =
            glyphon::TextAtlas::new(device, queue, &cache, color_format);
        let viewport = glyphon::Viewport::new(device, &cache);
        let swash_cache = glyphon::SwashCache::new();

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
            text_renderer,
            font_system,
            viewport,
            swash_cache,
        }
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
    ) -> Result<(), TextDrawError> {
        self.text_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.viewport,
            [],
            &mut self.swash_cache,
        )?;
        self.text_renderer.render(
            &self.text_atlas,
            &self.viewport,
            render_pass,
        )?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TextDrawError {
    #[error(transparent)]
    Prepare(#[from] glyphon::PrepareError),

    #[error(transparent)]
    Render(#[from] glyphon::RenderError),
}
