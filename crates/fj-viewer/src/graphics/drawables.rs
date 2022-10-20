use super::{
    geometries::{Geometries, Geometry},
    pipelines::{Pipeline, Pipelines},
};

pub struct Drawables<'r> {
    pub model: Drawable<'r>,
    pub mesh: Drawable<'r>,
    pub lines: Drawable<'r>,
}

impl<'r> Drawables<'r> {
    pub fn new(geometries: &'r Geometries, pipelines: &'r Pipelines) -> Self {
        let model = Drawable::new(&geometries.mesh, &pipelines.model);
        let mesh = Drawable::new(&geometries.mesh, &pipelines.mesh);
        let lines = Drawable::new(&geometries.lines, &pipelines.lines);

        Self { model, mesh, lines }
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

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        color_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
    ) {
        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        }),
                        stencil_ops: None,
                    },
                ),
            });

        render_pass.set_pipeline(&self.pipeline.0);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.geometry.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(0..self.geometry.num_indices, 0, 0..1);
    }
}
