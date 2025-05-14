use std::mem::size_of;

use super::{
    DEPTH_FORMAT, DrawConfig, SAMPLE_COUNT,
    geometry::Geometry,
    shaders::{Shader, Shaders},
    vertices::Vertex,
};

#[derive(Debug)]
pub struct Pipelines {
    pub model: Pipeline,
    pub mesh: Option<Pipeline>,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        color_format: wgpu::TextureFormat,
        features: wgpu::Features,
    ) -> Self {
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let shaders = Shaders::new(device);

        let model = Pipeline::new(
            device,
            &pipeline_layout,
            shaders.model(),
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::PolygonMode::Fill,
            color_format,
        );

        let mesh = if features.contains(wgpu::Features::POLYGON_MODE_LINE) {
            // We need this feature, otherwise initializing the pipeline will
            // panic.

            Some(Pipeline::new(
                device,
                &pipeline_layout,
                shaders.mesh(),
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::PolygonMode::Line,
                color_format,
            ))
        } else {
            None
        };

        Self { model, mesh }
    }

    pub fn draw(
        &self,
        config: &DrawConfig,
        geometry: &Geometry,
        render_pass: &mut wgpu::RenderPass,
    ) {
        if config.draw_model {
            self.model.draw(geometry, render_pass);
        }

        if let Some(pipeline) = self.mesh.as_ref() {
            if config.draw_mesh {
                pipeline.draw(geometry, render_pass);
            }
        };
    }
}

#[derive(Debug)]
pub struct Pipeline {
    pub inner: wgpu::RenderPipeline,
}

impl Pipeline {
    fn new(
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        shader: Shader,
        topology: wgpu::PrimitiveTopology,
        polygon_mode: wgpu::PolygonMode,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(pipeline_layout),
                vertex: wgpu::VertexState {
                    module: shader.module,
                    entry_point: Some("vertex"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float32x3,
                            1 => Float32x3,
                            2 => Float32x4,
                        ],
                    }],
                },
                primitive: wgpu::PrimitiveState {
                    topology,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState {
                        front: wgpu::StencilFaceState::IGNORE,
                        back: wgpu::StencilFaceState::IGNORE,
                        read_mask: 0,
                        write_mask: 0,
                    },
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: SAMPLE_COUNT,
                    mask: !0,
                    alpha_to_coverage_enabled: true,
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader.module,
                    entry_point: Some(shader.frag_entry),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: color_format,
                        blend: Some(
                            wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
                        ),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        Self { inner: pipeline }
    }

    pub fn draw(
        &self,
        geometry: &Geometry,
        render_pass: &mut wgpu::RenderPass,
    ) {
        render_pass.set_pipeline(&self.inner);
        render_pass.set_vertex_buffer(0, geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            geometry.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(0..geometry.num_indices, 0, 0..1);
    }
}
