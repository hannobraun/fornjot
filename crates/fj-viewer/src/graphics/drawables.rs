use super::{
    DrawConfig,
    geometry::Geometry,
    pipelines::{Pipeline, Pipelines},
};

pub struct Drawables<'a> {
    pub model: Drawable<'a>,
    pub mesh: Option<Drawable<'a>>,
}

impl<'a> Drawables<'a> {
    pub fn new(geometry: &'a Geometry, pipelines: &'a Pipelines) -> Self {
        let model = Drawable::new(geometry, &pipelines.model);
        let mesh = pipelines
            .mesh
            .as_ref()
            .map(|pipeline| Drawable::new(geometry, pipeline));

        Self { model, mesh }
    }

    pub fn draw<'b>(
        self,
        config: &DrawConfig,
        render_pass: &mut wgpu::RenderPass<'b>,
    ) where
        'a: 'b,
    {
        if config.draw_model {
            self.model.draw(render_pass);
        }

        if let Some(drawable) = self.mesh {
            if config.draw_mesh {
                drawable.draw(render_pass);
            }
        }
    }
}

pub struct Drawable<'a> {
    pub geometry: &'a Geometry,
    pub pipeline: &'a Pipeline,
}

impl<'a> Drawable<'a> {
    fn new(geometry: &'a Geometry, pipeline: &'a Pipeline) -> Self {
        Self { geometry, pipeline }
    }

    pub fn draw<'b>(&self, render_pass: &mut wgpu::RenderPass<'b>)
    where
        'a: 'b,
    {
        render_pass.set_pipeline(&self.pipeline.inner);
        render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.geometry.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(0..self.geometry.num_indices, 0, 0..1);
    }
}
