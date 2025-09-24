use glyphon::{FontSystem, TextArea, TextBounds};
use winit::dpi::PhysicalSize;

use crate::viewer::graphics::{DEPTH_FORMAT, MULTISAMPLE_STATE};

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
        _: PhysicalSize<u32>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let cache = glyphon::Cache::new(device);
        let mut text_atlas =
            glyphon::TextAtlas::new(device, queue, &cache, color_format);

        let depth_stencil = wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        let text_renderer = glyphon::TextRenderer::new(
            &mut text_atlas,
            device,
            MULTISAMPLE_STATE,
            Some(depth_stencil),
        );

        let mut font_system = FontSystem::new();
        let viewport = glyphon::Viewport::new(device, &cache);

        let mut text_buffer = glyphon::Buffer::new(
            &mut font_system,
            glyphon::Metrics::new(32., 32.),
        );
        text_buffer.set_size(&mut font_system, None, None);
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
        surface_config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass,
    ) -> Result<(), TextDrawError> {
        let text_areas = [TextArea {
            buffer: &self.text_buffer,
            left: 0.,
            top: 0.,
            scale: 1.,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: surface_config.width as i32,
                bottom: surface_config.height as i32,
            },
            default_color: glyphon::Color::rgb(0, 0, 0),
            custom_glyphs: &[],
        }];

        self.viewport.update(
            queue,
            glyphon::Resolution {
                width: surface_config.width,
                height: surface_config.height,
            },
        );
        self.text_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.viewport,
            text_areas,
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
