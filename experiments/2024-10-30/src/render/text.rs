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
        scale_factor: f32,
    ) -> Self {
        let cache = glyphon::Cache::new(device);
        let mut swash_cache = glyphon::SwashCache::new();

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

        let mut text_renderer = glyphon::TextRenderer::new(
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

        let mut font_system = glyphon::FontSystem::new();

        let mut buffer = glyphon::Buffer::new(
            &mut font_system,
            glyphon::Metrics {
                font_size: 16.,
                line_height: 16.,
            },
        );
        buffer.set_text(
            &mut font_system,
            "Hello, world!",
            glyphon::Attrs::new(),
            glyphon::Shaping::Advanced,
        );

        text_renderer
            .prepare(
                device,
                queue,
                &mut font_system,
                &mut text_atlas,
                &viewport,
                [glyphon::TextArea {
                    buffer: &buffer,
                    left: 0.,
                    top: 0.,
                    scale: scale_factor,
                    bounds: glyphon::TextBounds {
                        left: 0,
                        top: 0,
                        right: surface_config.width as i32,
                        bottom: surface_config.height as i32,
                    },
                    default_color: glyphon::Color::rgb(0, 0, 0),
                    custom_glyphs: &[],
                }],
                &mut swash_cache,
            )
            .unwrap();

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
