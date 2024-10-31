use std::{
    f32::consts::PI,
    ops::{Mul, Sub},
    sync::Arc,
};

use anyhow::anyhow;
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
                contents: bytemuck::cast_slice(
                    &Mat4x4::default_transform().columns,
                ),
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
                        entry_point: "vertex",
                        compilation_options:
                            wgpu::PipelineCompilationOptions::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: size_of::<Vertex>()
                                as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x3,
                                offset: 0,
                                shader_location: 0,
                            }],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fragment",
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
            let triangle = triangle.map(|index| Point {
                coords: mesh.vertices()[index as usize],
            });

            for point in triangle {
                let index = vertices.len() as u32;
                let vertex = Vertex {
                    position: point.coords,
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

struct Mat4x4 {
    columns: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn default_transform() -> Self {
        Self::perspective()
            * Self::translation([0., 0., -2.])
            * Self::rotation_x(PI / 4.)
            * Self::rotation_z(PI / 4.)
    }

    pub fn perspective() -> Self {
        let fov_y_radians = std::f32::consts::PI / 2.;
        let aspect_ratio = 1.;
        let z_near = 0.1;
        let z_far = 10.;

        let (sin_fov, cos_fov) = (fov_y_radians * 0.5).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);

        Self {
            columns: [
                [w, 0., 0., 0.],
                [0., h, 0., 0.],
                [0., 0., r, -1.],
                [0., 0., r * z_near, 0.],
            ],
        }
    }

    pub fn translation([x, y, z]: [f32; 3]) -> Self {
        Self {
            columns: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [x, y, z, 1.],
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self {
            columns: [
                [1., 0., 0., 0.],
                [0., cos, -sin, 0.],
                [0., sin, cos, 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self {
            columns: [
                [cos, -sin, 0., 0.],
                [sin, cos, 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}

impl Mul<Self> for Mat4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let [[l00, l01, l02, l03], [l10, l11, l12, l13], [l20, l21, l22, l23], [l30, l31, l32, l33]] =
            self.columns;
        let [[r00, r01, r02, r03], [r10, r11, r12, r13], [r20, r21, r22, r23], [r30, r31, r32, r33]] =
            rhs.columns;

        let m00 = l00 * r00 + l10 * r01 + l20 * r02 + l30 * r03;
        let m01 = l01 * r00 + l11 * r01 + l21 * r02 + l31 * r03;
        let m02 = l02 * r00 + l12 * r01 + l22 * r02 + l32 * r03;
        let m03 = l03 * r00 + l13 * r01 + l23 * r02 + l33 * r03;

        let m10 = l00 * r10 + l10 * r11 + l20 * r12 + l30 * r13;
        let m11 = l01 * r10 + l11 * r11 + l21 * r12 + l31 * r13;
        let m12 = l02 * r10 + l12 * r11 + l22 * r12 + l32 * r13;
        let m13 = l03 * r10 + l13 * r11 + l23 * r12 + l33 * r13;

        let m20 = l00 * r20 + l10 * r21 + l20 * r22 + l30 * r23;
        let m21 = l01 * r20 + l11 * r21 + l21 * r22 + l31 * r23;
        let m22 = l02 * r20 + l12 * r21 + l22 * r22 + l32 * r23;
        let m23 = l03 * r20 + l13 * r21 + l23 * r22 + l33 * r23;

        let m30 = l00 * r30 + l10 * r31 + l20 * r32 + l30 * r33;
        let m31 = l01 * r30 + l11 * r31 + l21 * r32 + l31 * r33;
        let m32 = l02 * r30 + l12 * r31 + l22 * r32 + l32 * r33;
        let m33 = l03 * r30 + l13 * r31 + l23 * r32 + l33 * r33;

        Self {
            columns: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub coords: [f32; 3],
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let [a_x, a_y, a_z] = self.coords;
        let [b_x, b_y, b_z] = rhs.coords;

        Vector {
            coords: [a_x - b_x, a_y - b_y, a_z - b_z],
        }
    }
}

pub struct Vector {
    #[allow(unused)]
    pub coords: [f32; 3],
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
}
