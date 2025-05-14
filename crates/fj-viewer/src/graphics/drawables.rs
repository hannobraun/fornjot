use super::{geometry::Geometry, pipelines::Pipeline};

pub struct Drawable<'a> {
    pub geometry: &'a Geometry,
    pub pipeline: &'a Pipeline,
}

impl<'a> Drawable<'a> {
    pub fn new(geometry: &'a Geometry, pipeline: &'a Pipeline) -> Self {
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
