use std::mem::size_of;

use super::{
    shaders::{Shader, Shaders},
    vertices::Vertex,
    DEPTH_FORMAT,
};

#[derive(Debug)]
pub struct Pipelines {
    pub model: Pipeline,
    pub mesh: Pipeline,
    pub lines: Pipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let shaders = Shaders::new(device);

        Self {
            model: Pipeline::new(
                device,
                &pipeline_layout,
                shaders.model(),
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::PolygonMode::Fill,
                color_format,
            ),
            mesh: Pipeline::new(
                device,
                &pipeline_layout,
                shaders.mesh(),
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::PolygonMode::Line,
                color_format,
            ),
            lines: Pipeline::new(
                device,
                &pipeline_layout,
                shaders.lines(),
                wgpu::PrimitiveTopology::LineList,
                wgpu::PolygonMode::Line,
                color_format,
            ),
        }
    }
}

#[derive(Debug)]
pub struct Pipeline(pub wgpu::RenderPipeline);

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
                    entry_point: "vertex",
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
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader.module,
                    entry_point: shader.frag_entry,
                    targets: &[Some(wgpu::ColorTargetState {
                        format: color_format,
                        blend: Some(
                            wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
                        ),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        Self(pipeline)
    }
}
