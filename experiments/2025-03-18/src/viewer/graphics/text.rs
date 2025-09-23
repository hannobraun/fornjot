use glyphon::{FontSystem, TextArea, TextBounds};
use winit::dpi::PhysicalSize;

pub struct TextRenderer {
    text_atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    font_system: glyphon::FontSystem,
    viewport: glyphon::Viewport,
    text_buffer: glyphon::Buffer,
    swash_cache: glyphon::SwashCache,
}

impl TextRenderer {
    pub fn new(
        display_size: PhysicalSize<u32>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let cache = glyphon::Cache::new(device);
        let mut text_atlas =
            glyphon::TextAtlas::new(device, queue, &cache, color_format);

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

        let mut font_system = FontSystem::new();
        let viewport = glyphon::Viewport::new(device, &cache);

        let mut text_buffer = glyphon::Buffer::new(
            &mut font_system,
            glyphon::Metrics::new(32., 32.),
        );
        text_buffer.set_size(
            &mut font_system,
            Some(display_size.width as f32),
            Some(display_size.height as f32),
        );
        text_buffer.set_text(
            &mut font_system,
            "Hello, world!",
            &glyphon::Attrs::new(),
            glyphon::Shaping::Advanced,
        );

        let swash_cache = glyphon::SwashCache::new();

        Self {
            text_atlas,
            text_renderer,
            font_system,
            viewport,
            text_buffer,
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
            [TextArea {
                buffer: &self.text_buffer,
                left: 0.,
                top: 0.,
                scale: 1.,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
                default_color: glyphon::Color::rgb(0, 0, 0),
                custom_glyphs: &[],
            }],
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
