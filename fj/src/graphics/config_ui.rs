use wgpu::util::StagingBelt;
use wgpu_glyph::{GlyphBrush, Section, Text};

pub struct ConfigUi;

impl ConfigUi {
    pub fn draw(
        &mut self,
        glyph_brush: &mut GlyphBrush<()>,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Result<(), String> {
        // TASK: Update this to display the current configuration. Ideas:
        //       - Display text like "X enabled/disabled (toggle with y)".
        //       - Make text for disabled config semi-transparent.
        glyph_brush.queue(
            Section::new()
                .with_screen_position((50.0, 50.0))
                .add_text(text("Toggle model rendering with 1\n", true))
                .add_text(text("Toggle mesh rendering with 2\n", true))
                .add_text(text("Toggle grid rendering with 3\n", true)),
        );

        glyph_brush.draw_queued(
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

fn text(text: &str, opaque: bool) -> Text {
    let alpha = if opaque { 1.0 } else { 0.75 };

    Text::new(text)
        .with_color([0.0, 0.0, 0.0, alpha])
        .with_scale(100.0)
}
