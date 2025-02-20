use fj_math::Transform;
use wgpu::util::DeviceExt;

use super::{
    model::{self, DrawModel, Model, load_model},
    transform,
};

#[derive(Debug)]
pub struct NavigationCubeRenderer {
    cube_model: Model,
    render_pipeline: wgpu::RenderPipeline,
    mvp_matrix_bind_group: wgpu::BindGroup,
    mvp_matrix_buffer: wgpu::Buffer,
}

const SCALE_FACTOR: f64 = 0.13;
const CUBE_TRANSLATION: [f64; 3] = [0.8, 0.7, 0.0];

impl NavigationCubeRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let mvp_matrix_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Model Matrix Buffer"),
                contents: bytemuck::cast_slice(&[
                    transform::Transform::identity(),
                ]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            });
        let mvp_matrix_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("mvp_matrix_group_layout"),
            });
        let mvp_matrix_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &mvp_matrix_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: mvp_matrix_buffer.as_entire_binding(),
                }],
                label: Some("mvp_matrix_bind_group"),
            });

        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shadow Display Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("navigation_cube.wgsl").into(),
                ),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &mvp_matrix_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Navigation Cube Renderer"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    buffers: &[model::ModelVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                    // or Features::POLYGON_MODE_POINT
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        let cube_model =
            load_model("cube.obj", device, queue, &texture_bind_group_layout)
                .unwrap();

        Self {
            cube_model,
            render_pipeline,
            mvp_matrix_bind_group,
            mvp_matrix_buffer,
        }
    }

    pub fn draw(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        aspect_ratio: f64,
        rotation: Transform,
    ) {
        let mvp_matrix = Self::get_mvp_matrix(rotation, aspect_ratio);
        queue.write_buffer(
            &self.mvp_matrix_buffer,
            0,
            bytemuck::cast_slice(&[mvp_matrix]),
        );

        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.mvp_matrix_bind_group, &[]);
        render_pass.draw_model(&self.cube_model);
    }

    fn get_mvp_matrix(rotation: Transform, aspect_ratio: f64) -> [f32; 16] {
        let scale = Transform::scale(SCALE_FACTOR);
        let world_translation = Transform::translation([0.0, 0.0, -1.0]);

        let mut model_matrix = Transform::identity();
        model_matrix = model_matrix * world_translation;
        model_matrix = model_matrix * rotation;
        model_matrix = model_matrix * scale;

        let perspective =
            nalgebra::Perspective3::new(aspect_ratio, 30.0, 0.1, 2.0);

        let view_matrix = nalgebra::Matrix4::look_at_lh(
            &nalgebra::Point3::new(0.0, 0.0, 0.0),
            &nalgebra::Point3::new(0.0, 0.0, 1.0),
            &nalgebra::Vector3::new(0.0, -1.0, 0.0),
        );

        let screen_translation = Transform::translation(CUBE_TRANSLATION);

        let matrix = screen_translation.get_inner().matrix()
            * *perspective.to_projective().matrix()
            * view_matrix
            * model_matrix.get_inner().matrix();

        let mut mat = [0.; 16];
        mat.copy_from_slice(matrix.as_slice());
        mat.map(|x| x as f32)
    }
}
