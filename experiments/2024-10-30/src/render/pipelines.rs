use super::{
    geometry::Geometry,
    shaders::{Shaders, Vertex},
};

pub struct Pipelines {
    pub vertices: Pipeline,
    pub triangles: Pipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let vertices = Pipeline::vertices(device, config, uniforms);
        let triangles = Pipeline::triangles(device, config, uniforms);

        Self {
            vertices,
            triangles,
        }
    }
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl Pipeline {
    pub fn vertices(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let vertices_shaders = Shaders::vertices(device, config);
        Pipeline::new(device, &vertices_shaders, uniforms)
    }

    pub fn triangles(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let triangles_shaders = Shaders::triangles(device, config);
        Pipeline::new(device, &triangles_shaders, uniforms)
    }

    pub fn new(
        device: &wgpu::Device,
        shaders: &Shaders<impl Vertex>,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&shaders.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: shaders.vertex_state(),
                fragment: Some(shaders.fragment_state()),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &shaders.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });

        Pipeline {
            render_pipeline,
            bind_group,
        }
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        color_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        geometry: &Geometry,
    ) {
        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        if geometry.num_indices > 0 {
            render_pass.set_index_buffer(
                geometry.indices.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.set_vertex_buffer(0, geometry.vertices.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw_indexed(0..geometry.num_indices, 0, 0..1);
        }
    }
}
