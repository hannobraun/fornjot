use wgpu::util::StagingBelt;
use wgpu_glyph::{
    ab_glyph::{FontArc, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

use super::COLOR_FORMAT;

#[derive(Debug)]
pub struct ConfigUi {
    glyph_brush: GlyphBrush<()>,

    model_text: String,
    mesh_text: String,
    grid_text: String,
}

impl ConfigUi {
    pub fn new(device: &wgpu::Device) -> Result<Self, InvalidFont> {
        let font =
            FontArc::try_from_slice(include_bytes!("fonts/B612-Bold.ttf"))?;
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(device, COLOR_FORMAT);

        let model_text = format!("Toggle model rendering with 1\n");
        let mesh_text = format!("Toggle mesh rendering with 2\n");
        let grid_text = format!("Toggle grid rendering with 3\n");

        Ok(Self {
            glyph_brush,
            model_text,
            mesh_text,
            grid_text,
        })
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Result<(), String> {
        // TASK: Update this to display the current configuration. Ideas:
        //       - Display text like "X enabled/disabled (toggle with y)".
        //       - Make text for disabled config semi-transparent.
        self.glyph_brush.queue(
            Section::new()
                .with_screen_position((50.0, 50.0))
                .add_text(self.text(&self.model_text, true))
                .add_text(self.text(&self.mesh_text, true))
                .add_text(self.text(&self.grid_text, true)),
        );

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

    fn text<'r>(&self, text: &'r str, opaque: bool) -> Text<'r> {
        let alpha = if opaque { 1.0 } else { 0.75 };

        Text::new(text)
            .with_color([0.0, 0.0, 0.0, alpha])
            .with_scale(100.0)
    }
}
