use std::collections::HashMap;

use wgpu::util::StagingBelt;
use wgpu_glyph::{
    ab_glyph::{FontArc, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

use super::COLOR_FORMAT;

#[derive(Debug)]
pub struct ConfigUi {
    glyph_brush: GlyphBrush<()>,
    texts: HashMap<Element, String>,
}

impl ConfigUi {
    pub fn new(device: &wgpu::Device) -> Result<Self, InvalidFont> {
        let font =
            FontArc::try_from_slice(include_bytes!("fonts/B612-Bold.ttf"))?;
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(device, COLOR_FORMAT);

        let mut texts = HashMap::new();
        texts
            .insert(Element::Model, format!("Toggle model rendering with 1\n"));
        texts.insert(Element::Mesh, format!("Toggle mesh rendering with 2\n"));
        texts.insert(Element::Grid, format!("Toggle grid rendering with 3\n"));

        Ok(Self { glyph_brush, texts })
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Result<(), String> {
        let mut section = Section::new().with_screen_position((50.0, 50.0));

        for element in Element::elements() {
            let text = &self.texts[&element];
            let opaque = true;

            let alpha = if opaque { 1.0 } else { 0.75 };

            let text = Text::new(text)
                .with_color([0.0, 0.0, 0.0, alpha])
                .with_scale(100.0);

            section = section.add_text(text);
        }

        // TASK: Update this to display the current configuration. Ideas:
        //       - Display text like "X enabled/disabled (toggle with y)".
        //       - Make text for disabled config semi-transparent.
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

#[derive(Debug, Eq, Hash, PartialEq)]
enum Element {
    Model,
    Mesh,
    Grid,
}

impl Element {
    fn elements() -> &'static [Self] {
        &[Self::Model, Self::Mesh, Self::Grid]
    }
}
