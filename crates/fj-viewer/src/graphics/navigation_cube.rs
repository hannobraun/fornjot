use cgmath::{Quaternion, Rotation3, Vector3};
use wgpu::util::DeviceExt;

use super::{
    model::{self, load_model, DrawModel, Model},
    texture,
};

#[derive(Debug)]
pub struct NavigationCubeRenderer {
    cube_model: Model,
    render_pipeline: wgpu::RenderPipeline,
    rotation: f32,
    model_matrix_bind_group: wgpu::BindGroup,
    model_matrix_buffer: wgpu::Buffer,
}

impl NavigationCubeRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        aspect_ratio: f64,
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

        let rotation = 0.0;
        let model_matrix = Self::get_model_matrix(rotation, &aspect_ratio);

        let model_matrix_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Model Matrix Buffer"),
                contents: bytemuck::cast_slice(&[model_matrix]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            });
        let model_matrix_bind_group_layout =
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
                label: Some("model_matrix_group_layout"),
            });
        let model_matrix_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &model_matrix_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_matrix_buffer.as_entire_binding(),
                }],
                label: Some("model_matrix_bind_group"),
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
                    &model_matrix_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Navigation Cube Renderer"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &&shader,
                    entry_point: "vertex",
                    buffers: &[model::ModelVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fragment",
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
            });

        let cube_model =
            load_model("cube.obj", &device, &queue, &texture_bind_group_layout)
                .unwrap();

        Self {
            cube_model,
            render_pipeline,
            rotation,
            model_matrix_bind_group,
            model_matrix_buffer,
        }
    }

    pub fn draw(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        aspect_ratio: &f64,
    ) {
        self.rotation += 0.5;
        let model_matrix = Self::get_model_matrix(self.rotation, aspect_ratio);
        queue.write_buffer(
            &self.model_matrix_buffer,
            0,
            bytemuck::cast_slice(&[model_matrix]),
        );

        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Depth Visual Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.model_matrix_bind_group, &[]);
        render_pass.draw_model(&self.cube_model);
    }

    fn get_model_matrix(rotation: f32, aspect_ratio: &f64) -> [[f32; 4]; 4] {
        let rotation = Quaternion::from_angle_y(cgmath::Deg(rotation));

        let scale = cgmath::Matrix4::from_nonuniform_scale(
            0.2,
            (0.2 * aspect_ratio) as f32,
            0.2,
        );

        let translation =
            cgmath::Matrix4::from_translation((0.8, 0.8, 0.0).into());
        (translation * cgmath::Matrix4::from(rotation) * scale).into()
    }
}
