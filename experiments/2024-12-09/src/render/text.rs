use std::fmt::Write;

use crate::{object::Object, view::OperationView};

pub struct TextRenderer {
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    scale_factor: f32,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
        scale_factor: f32,
    ) -> Self {
        let cache = glyphon::Cache::new(device);
        let swash_cache = glyphon::SwashCache::new();

        let mut text_atlas = glyphon::TextAtlas::new(
            device,
            queue,
            &cache,
            surface_config.format,
        );

        let mut viewport = glyphon::Viewport::new(device, &cache);
        viewport.update(
            queue,
            glyphon::Resolution {
                width: surface_config.width,
                height: surface_config.height,
            },
        );

        let text_renderer = glyphon::TextRenderer::new(
            &mut text_atlas,
            device,
            wgpu::MultisampleState::default(),
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        let font_system = glyphon::FontSystem::new();

        Self {
            text_atlas,
            viewport,
            text_renderer,
            font_system,
            swash_cache,
            scale_factor,
        }
    }

    pub fn render(
        &mut self,
        operations: &OperationView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass,
    ) -> anyhow::Result<()> {
        let mut buffer = glyphon::Buffer::new(
            &mut self.font_system,
            glyphon::Metrics {
                font_size: 16.,
                line_height: 16.,
            },
        );

        for (op, selected, indent_level) in operations.operations() {
            let mut attrs = glyphon::Attrs::new();

            if selected {
                attrs = attrs.color(glyphon::Color::rgb(0, 127, 0));
            }

            let mut line = String::new();

            for _ in 0..indent_level {
                write!(line, "\t")?;
            }

            write!(line, "{}", op.label())?;

            buffer.lines.push(glyphon::BufferLine::new(
                line,
                glyphon::cosmic_text::LineEnding::Lf,
                glyphon::AttrsList::new(attrs),
                glyphon::Shaping::Advanced,
            ));
        }

        buffer.shape_until_scroll(&mut self.font_system, false);

        let text_area = glyphon::TextArea {
            buffer: &buffer,
            left: 0.,
            top: 0.,
            scale: self.scale_factor,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: surface_config.width as i32,
                bottom: surface_config.height as i32,
            },
            default_color: glyphon::Color::rgb(0, 0, 0),
            custom_glyphs: &[],
        };

        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &self.viewport,
                [text_area],
                &mut self.swash_cache,
            )
            .unwrap();

        self.text_renderer.render(
            &self.text_atlas,
            &self.viewport,
            render_pass,
        )?;

        Ok(())
    }
}
