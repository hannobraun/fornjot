use std::collections::HashMap;

use fj_math::Aabb;
use wgpu::util::StagingBelt;
use wgpu_glyph::{
    ab_glyph::{FontArc, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

use super::draw_config::DrawConfig;

pub struct ConfigUi {
    glyph_brush: GlyphBrush<()>,
    texts: HashMap<(Element, bool), String>,
    staging_belt: StagingBelt,
}

impl std::fmt::Debug for ConfigUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigUi")
            .field("glyph_brush", &self.glyph_brush)
            .field("texts", &self.texts)
            .finish()
    }
}

impl ConfigUi {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
    ) -> Result<Self, InvalidFont> {
        let font =
            FontArc::try_from_slice(include_bytes!("fonts/B612-Bold.ttf"))?;
        let glyph_brush = GlyphBrushBuilder::using_font(font)
            .initial_cache_size((512, 512))
            .build(device, color_format);

        let mut texts = HashMap::new();
        for element in Element::elements() {
            let (name, key) = element.name_key();

            for (enabled, word) in [(false, "disabled"), (true, "enabled")] {
                texts.insert(
                    (element, enabled),
                    format!(
                        "{} rendering is {} (toggle with {})\n",
                        name, word, key
                    ),
                );
            }
        }

        // I haven't put any thought into the staging belt's buffer size.
        // 1024 just seemed like a good number, and so far it hasn't caused
        // any problems.
        //
        // - @hannobraun
        let staging_belt = StagingBelt::new(1024);

        Ok(Self {
            glyph_brush,
            texts,
            staging_belt,
        })
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        surface_config: &wgpu::SurfaceConfiguration,
        aabb: &Aabb<3>,
        draw_config: &DrawConfig,
    ) -> Result<(), String> {
        let mut section = Section::new().with_screen_position((50.0, 50.0));

        for element in Element::elements() {
            let enabled = element.is_enabled(draw_config);
            let text = &self.texts[&(element, enabled)];

            let alpha = if enabled { 1.0 } else { 0.75 };

            let text = Text::new(text)
                .with_color([0.0, 0.0, 0.0, alpha])
                .with_scale(50.0);

            section = section.add_text(text);
        }

        /* Render size of model bounding box */
        let bbsize = aabb.size().components;
        let info = format!(
            "Model bounding box size: {:0.1} {:0.1} {:0.1}",
            bbsize[0].into_f32(),
            bbsize[1].into_f32(),
            bbsize[2].into_f32()
        );
        let text = Text::new(&info)
            .with_color([0.0, 0.0, 0.0, 1.0])
            .with_scale(50.0);
        section = section.add_text(text);

        self.glyph_brush.queue(section);
        self.glyph_brush.draw_queued(
            device,
            &mut self.staging_belt,
            encoder,
            view,
            surface_config.width,
            surface_config.height,
        )?;

        self.staging_belt.finish();

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Element {
    Model,
    Mesh,
    Debug,
}

impl Element {
    fn elements() -> [Self; 3] {
        [Self::Model, Self::Mesh, Self::Debug]
    }

    fn name_key(&self) -> (&'static str, &'static str) {
        match self {
            Self::Model => ("model", "1"),
            Self::Mesh => ("mesh", "2"),
            Self::Debug => ("debug info", "3"),
        }
    }

    fn is_enabled(&self, config: &DrawConfig) -> bool {
        match self {
            Self::Model => config.draw_model,
            Self::Mesh => config.draw_mesh,
            Self::Debug => config.draw_debug,
        }
    }
}
