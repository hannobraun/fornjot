use std::{borrow::Cow, convert::TryInto, io, mem::size_of};

use tracing::debug;
use wgpu::util::DeviceExt as _;
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    mesh::Mesh, transform::Transform, uniforms::Uniforms, vertices::Vertex,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,

    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,

    num_indices: u32,
}

impl Renderer {
    pub async fn new(window: &Window, mesh: Mesh) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

        // This is sound, as `window` is an object to create a surface upon.
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(InitError::RequestAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // TASK: Be smarter about this. Any feature enabled here
                    //       might lead to a runtime error.
                    //
                    //       It might be best to request a device for every
                    //       single feature that is desired, log a warning for
                    //       each that isn't available, then request the final
                    //       device for those that are.
                    //
                    //       In addition, the available features must be stored
                    //       somewhere, so code that requires any unavailable
                    //       ones isn't run.
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(InitError::RequestDevice)?;

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::default()]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            });
        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.indices()),
                usage: wgpu::BufferUsages::INDEX,
            });

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(size_of::<
                            Uniforms,
                        >(
                        )
                            as u64),
                    },
                    count: None,
                }],
                label: None,
            });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: None,
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shader.wgsl"
                ))),
            });

        let (depth_texture, depth_view) =
            create_depth_buffer(&device, &surface_config);

        let render_pipeline =
            create_render_pipeline(&device, &pipeline_layout, &shader);

        Ok(Self {
            surface,
            device,
            queue,

            surface_config,

            uniform_buffer,
            vertex_buffer,
            index_buffer,

            depth_texture,
            depth_view,

            bind_group,
            render_pipeline,

            num_indices: mesh
                .indices()
                .len()
                .try_into()
                .expect("`usize` couldn't be cast to `u32`"),
        })
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);

        let (depth_texture, depth_view) =
            create_depth_buffer(&self.device, &self.surface_config);
        self.depth_texture = depth_texture;
        self.depth_view = depth_view;
    }

    pub fn draw(&mut self, transform: &Transform) -> Result<(), DrawError> {
        let uniforms = Uniforms {
            transform: transform.to_native(self.aspect_ratio()),
            transform_normals: transform.to_normals_transform(),
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let output =
            self.surface.get_current_frame().map_err(DrawError)?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: &self.depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        },
                    ),
                });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // Workaround for wgpu#1797:
        // https://github.com/gfx-rs/wgpu/issues/1797
        drop(view);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));

        debug!("Dropping `output`...");
        drop(output);

        debug!("Finished drawing.");
        Ok(())
    }

    fn aspect_ratio(&self) -> f32 {
        self.surface_config.width as f32 / self.surface_config.height as f32
    }
}

fn create_depth_buffer(
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, view)
}

fn create_render_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x3,
                    1 => Float32x3
                ],
            }],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            clamp_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
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
            module: &shader,
            entry_point: "fragment",
            targets: &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
    })
}

#[derive(Debug)]
pub enum InitError {
    Io(io::Error),
    RequestAdapter,
    RequestDevice(wgpu::RequestDeviceError),
}

impl From<io::Error> for InitError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug)]
pub struct DrawError(pub wgpu::SurfaceError);
