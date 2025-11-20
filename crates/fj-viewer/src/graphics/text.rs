use fj_math::{Point, Vector};
use glyphon::{FontSystem, TextArea, TextBounds};

use crate::graphics::{DEPTH_FORMAT, MULTISAMPLE_STATE, transform::Transform};

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

        let font_system = FontSystem::new();
        let viewport = glyphon::Viewport::new(device, &cache);

        let swash_cache = glyphon::SwashCache::new();

        Self {
            text_atlas,
            text_renderer,
            font_system,
            viewport,
            swash_cache,
        }
    }

    #[must_use]
    pub fn make_label(
        &mut self,
        text: &str,
        position: impl Into<Point<3>>,
    ) -> Label {
        let position = position.into();

        let mut buffer = glyphon::Buffer::new(
            &mut self.font_system,
            glyphon::Metrics::new(24., 24.),
        );
        buffer.set_text(
            &mut self.font_system,
            text,
            &glyphon::Attrs::new(),
            glyphon::Shaping::Advanced,
        );

        Label { buffer, position }
    }

    pub fn draw<'r>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
        render_pass: &mut wgpu::RenderPass,
        labels: impl IntoIterator<Item = &'r Label>,
        transform: &Transform,
    ) -> Result<(), TextDrawError> {
        let text_areas = labels.into_iter().map(|label| {
            let screen_position = {
                let mut point =
                    transform.inner().transform_point(&label.position);

                // The transform above has transformed the point into normalized
                // device coordinates, but we need pixel coordinates. Let's
                // start with moving the coordinate system origin to the upper-
                // left corner.
                point += Vector::from([1., -1., 0.]);

                // Normalized device coordinates cover the range from -1 to 1.
                // Before we can multiply that with the screen size, we need to
                // get a range from 0 to 1. While we're at that, also invert the
                // y-axis, to match the pixel coordinate system that we need.
                point.x *= 0.5;
                point.y *= -0.5;

                // At this point, we've transformed the position into a
                // normalized coordinate system (with range 0 to 1) with the
                // origin in the upper-left corner. All that's left is to
                // multiply by the screen size, and we have pixel coordinates.
                point.x *= surface_config.width as f64;
                point.y *= surface_config.height as f64;

                point
            };

            TextArea {
                buffer: &label.buffer,
                left: screen_position.x.into_f32(),
                top: screen_position.y.into_f32(),
                scale: 1.,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: surface_config.width as i32,
                    bottom: surface_config.height as i32,
                },
                default_color: glyphon::Color::rgb(0, 0, 0),
                custom_glyphs: &[],
            }
        });

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

#[derive(Debug)]
pub struct Label {
    position: Point<3>,
    buffer: glyphon::Buffer,
}

#[derive(Debug, thiserror::Error)]
pub enum TextDrawError {
    #[error(transparent)]
    Prepare(#[from] glyphon::PrepareError),

    #[error(transparent)]
    Render(#[from] glyphon::RenderError),
}
