use std::{f32::consts::PI, sync::Arc};

use anyhow::anyhow;
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::mesh::Mesh;

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or_else(|| anyhow!("Failed to request adapter"))?;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| anyhow!("Failed to get default surface config"))?;
        surface.configure(&device, &config);

        let transform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::default()]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let (pipeline, bind_group) = {
            let bind_group_layout = device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: None,
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
                },
            );

            let layout = device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                },
            );

            let shader =
                device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

            let pipeline = device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vertex"),
                        compilation_options:
                            wgpu::PipelineCompilationOptions::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: size_of::<Vertex>()
                                as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![
                                0 => Float32x3,
                                1 => Float32x3,
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fragment"),
                        compilation_options:
                            wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::all(),
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                    cache: None,
                },
            );

            let bind_group =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transform_buffer.as_entire_binding(),
                    }],
                });

            (pipeline, bind_group)
        };

        Ok(Self {
            surface,
            device,
            queue,
            pipeline,
            bind_group,
        })
    }

    pub fn render(&self, mesh: &Mesh) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut indices = Vec::new();
        let mut vertices = Vec::new();

        for triangle in mesh.triangles() {
            let triangle = triangle
                .map(|index| Vec3::from(mesh.vertices()[index as usize]));
            let normal = {
                let [a, b, c] = triangle;

                let ab = b - a;
                let ac = c - a;

                Vec3::new(
                    ab.y * ac.z - ab.z * ac.y,
                    ab.z * ac.x - ab.x * ac.z,
                    ab.x * ac.y - ab.z * ac.x,
                )
            };

            for point in triangle {
                let index = vertices.len() as u32;
                let vertex = Vertex {
                    position: point.into(),
                    normal: normal.into(),
                };

                indices.push(index);
                vertices.push(vertex);
            }
        }

        let index_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                });
        let vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    ..Default::default()
                });

            render_pass.set_index_buffer(
                index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw_indexed(
                0..mesh.triangles().len() as u32 * 3,
                0,
                0..1,
            );
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: Mat4,
}

impl Default for Uniforms {
    fn default() -> Self {
        let transform = default_transform();
        Self { transform }
    }
}

fn default_transform() -> Mat4 {
    let fov_y_radians = std::f32::consts::PI / 2.;
    let aspect_ratio = 1.;
    let z_near = 0.1;
    let z_far = 10.;

    Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)
        * Mat4::from_translation(Vec3::new(0., 0., -2.))
        * Mat4::from_rotation_x(-PI / 4.)
        * Mat4::from_rotation_z(PI / 4.)
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
