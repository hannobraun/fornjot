use std::collections::HashMap;

use wgpu::util::StagingBelt;
use wgpu_glyph::{
    ab_glyph::{FontArc, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

use super::{draw_config::DrawConfig, COLOR_FORMAT};

#[derive(Debug)]
pub struct ConfigUi {
    glyph_brush: GlyphBrush<()>,
    texts: HashMap<(Element, bool), String>,
}

impl ConfigUi {
    pub fn new(device: &wgpu::Device) -> Result<Self, InvalidFont> {
        let font =
            FontArc::try_from_slice(include_bytes!("fonts/B612-Bold.ttf"))?;
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(device, COLOR_FORMAT);

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

        Ok(Self { glyph_brush, texts })
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        surface_config: &wgpu::SurfaceConfiguration,
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

        self.glyph_brush.queue(section);
        self.glyph_brush.draw_queued(
            device,
            // TASK: Put more thought into the staging belt's buffer size.
            &mut StagingBelt::new(1024),
            encoder,
            &view,
            surface_config.width,
            surface_config.height,
        )?;

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
