use super::{geometry::Geometry, pipelines::Pipeline};

pub struct Drawable;

impl Drawable {
    pub fn draw(
        &self,
        geometry: &Geometry,
        pipeline: &Pipeline,
        render_pass: &mut wgpu::RenderPass,
    ) {
        render_pass.set_pipeline(&pipeline.inner);
        render_pass.set_vertex_buffer(0, geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            geometry.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(0..geometry.num_indices, 0, 0..1);
    }
}
