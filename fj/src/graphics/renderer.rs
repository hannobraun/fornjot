use std::{convert::TryInto, io, mem::size_of};

use tracing::{instrument, trace};
use wgpu::util::DeviceExt as _;
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    mesh::Mesh,
    shaders::{self, Shaders},
    transform::Transform,
    uniforms::Uniforms,
    vertices::Vertex,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

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
        let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);

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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|err| InitError::RequestDevice(err))?;

        let size = window.inner_size();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::default()]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });
        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.vertices()),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.indices()),
                usage: wgpu::BufferUsage::INDEX,
            });

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
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

        let shaders =
            Shaders::compile().map_err(|err| InitError::Shaders(err))?;

        let vertex_shader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::util::make_spirv(shaders.vertex.as_binary_u8()),
                flags: wgpu::ShaderFlags::VALIDATION,
            });
        let fragment_shader =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::util::make_spirv(shaders.fragment.as_binary_u8()),
                flags: wgpu::ShaderFlags::VALIDATION,
            });

        let (depth_texture, depth_view) =
            create_depth_buffer(&device, &swap_chain_desc);

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as u64,
                        step_mode: wgpu::InputStepMode::Vertex,
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
                    module: &fragment_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        blend: None,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
            });

        Ok(Self {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,

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
        self.swap_chain_desc.width = size.width;
        self.swap_chain_desc.height = size.height;

        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);

        let (depth_texture, depth_view) =
            create_depth_buffer(&self.device, &self.swap_chain_desc);
        self.depth_texture = depth_texture;
        self.depth_view = depth_view;
    }

    #[instrument]
    pub fn draw(&mut self, transform: &Transform) -> Result<(), DrawError> {
        trace!("Drawing...");

        let uniforms = Uniforms {
            transform: transform.to_native(self.aspect_ratio()),
            transform_normals: transform.to_normals_transform(),
        };

        self.queue.write_buffer(
            &mut self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let output = self
            .swap_chain
            .get_current_frame()
            .map_err(|err| DrawError(err))?
            .output;

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &output.view,
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

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    fn aspect_ratio(&self) -> f32 {
        self.swap_chain_desc.width as f32 / self.swap_chain_desc.height as f32
    }
}

fn create_depth_buffer(
    device: &wgpu::Device,
    swap_chain_desc: &wgpu::SwapChainDescriptor,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: swap_chain_desc.width,
            height: swap_chain_desc.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, view)
}

#[derive(Debug)]
pub enum InitError {
    Io(io::Error),
    RequestAdapter,
    RequestDevice(wgpu::RequestDeviceError),
    Shaders(shaders::Error),
}

impl From<io::Error> for InitError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug)]
pub struct DrawError(pub wgpu::SwapChainError);
