// TASK: Implement a way to display the grid that is used to sample the
//       geometry.

use std::{io, mem::size_of};

use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    drawables::Drawable, geometry::Geometries, mesh::Mesh,
    pipelines::Pipelines, transform::Transform, uniforms::Uniforms,
    DEPTH_FORMAT,
};

#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,
    depth_view: wgpu::TextureView,

    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    geometries: Geometries,
    pipelines: Pipelines,

    draw_model: bool,
    draw_mesh: bool,
    draw_grid: bool,
}

impl Renderer {
    pub async fn new(window: &Window, mesh: &Mesh) -> Result<Self, InitError> {
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
            .await?;

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let depth_view = Self::create_depth_buffer(&device, &surface_config);

        let uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::default()]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
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

        let geometries = Geometries::new(&device, mesh);
        let pipelines = Pipelines::new(&device, &bind_group_layout);

        Ok(Self {
            surface,
            device,
            queue,

            surface_config,
            depth_view,

            uniform_buffer,
            bind_group,

            geometries,
            pipelines,

            draw_model: true,
            draw_mesh: false,
            draw_grid: false,
        })
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);

        let depth_view =
            Self::create_depth_buffer(&self.device, &self.surface_config);
        self.depth_view = depth_view;
    }

    pub fn toggle_model(&mut self) {
        self.draw_model = !self.draw_model;
    }

    pub fn toggle_mesh(&mut self) {
        self.draw_mesh = !self.draw_mesh;
    }

    // TASK: This doesn't currently do anything.
    pub fn toggle_grid(&mut self) {
        self.draw_grid = !self.draw_grid;
    }

    pub fn draw(&mut self, transform: &Transform) -> Result<(), DrawError> {
        let uniforms = Uniforms {
            transform: transform.to_native(self.aspect_ratio()),
            transform_normals: transform.to_normals_transform(),
            ..Uniforms::default()
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let output = self.surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        self.clear_views(&mut encoder, &view);

        if self.draw_model {
            let model =
                Drawable::new(&self.geometries.mesh, &self.pipelines.model);
            model.draw(&mut encoder, &view, &self.depth_view, &self.bind_group);
        }
        if self.draw_mesh {
            let mesh =
                Drawable::new(&self.geometries.mesh, &self.pipelines.mesh);
            mesh.draw(&mut encoder, &view, &self.depth_view, &self.bind_group);
        }

        // Workaround for gfx-rs/wgpu#1797:
        // https://github.com/gfx-rs/wgpu/issues/1797
        drop(view);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));

        debug!("Dropping `output`...");
        drop(output);

        debug!("Finished drawing.");
        Ok(())
    }

    fn create_depth_buffer(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
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

        view
    }

    fn aspect_ratio(&self) -> f32 {
        self.surface_config.width as f32 / self.surface_config.height as f32
    }

    fn clear_views(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
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
    }
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Error request adapter")]
    RequestAdapter,

    #[error("Error requesting device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}

#[derive(Error, Debug)]
#[error("Draw error")]
pub struct DrawError(#[from] pub wgpu::SurfaceError);
