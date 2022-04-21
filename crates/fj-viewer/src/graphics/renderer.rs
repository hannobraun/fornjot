use std::{io, mem::size_of};

use fj_math::{Aabb, Point};
use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;
use wgpu_glyph::ab_glyph::InvalidFont;
use winit::dpi::PhysicalSize;

use crate::{camera::Camera, window::Window};

use super::{
    config_ui::ConfigUi, draw_config::DrawConfig, drawables::Drawables,
    geometries::Geometries, pipelines::Pipelines, transform::Transform,
    uniforms::Uniforms, vertices::Vertices, DEPTH_FORMAT,
};

#[derive(Debug)]
/// Graphics rendering state and target abstraction.
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

    config_ui: ConfigUi,
}

impl Renderer {
    /// Returns a new `Renderer`
    ///
    /// # Arguments
    /// - `window` - a `crate::window::Window` with a surface to render onto.
    ///
    /// # Examples
    /// ```rust no_run
    /// use fj_viewer::{graphics, window};
    ///
    /// // Create window
    /// let event_loop = winit::event_loop::EventLoop::new();
    /// let window = window::Window::new(&event_loop);
    ///
    /// // Attach renderer to the window
    /// let mut renderer = graphics::Renderer::new(&window);
    /// ```
    pub async fn new(window: &Window) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        // This is sound, as `window` is an object to create a surface upon.
        let surface = unsafe { instance.create_surface(window.inner()) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(InitError::RequestAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // Don't just blindly assume that we can request this
                    // feature. If it isn't available, that might cause a panic,
                    // or an error to be returned here.
                    //
                    // See this issue:
                    // https://github.com/hannobraun/fornjot/issues/33
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let color_format = surface
            .get_preferred_format(&adapter)
            .expect("Error determining preferred color format");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: color_format,
            width: window.width(),
            height: window.height(),
            present_mode: wgpu::PresentMode::Mailbox,
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

        let geometries = Geometries::new(
            &device,
            &Vertices::empty(),
            &Vertices::empty(),
            Aabb {
                min: Point::from([0.0, 0.0, 0.0]),
                max: Point::from([0.0, 0.0, 0.0]),
            },
        );
        let pipelines =
            Pipelines::new(&device, &bind_group_layout, color_format);

        let config_ui = ConfigUi::new(&device, color_format)?;

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

            config_ui,
        })
    }

    /// Updates the geometry of the model being rendered.
    pub fn update_geometry(
        &mut self,
        mesh: Vertices,
        lines: Vertices,
        aabb: Aabb<3>,
    ) {
        self.geometries = Geometries::new(&self.device, &mesh, &lines, aabb);
    }

    /// Resizes the render surface
    ///
    /// # Arguments
    /// - `size`: The target size for the render surface.
    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);

        let depth_view =
            Self::create_depth_buffer(&self.device, &self.surface_config);
        self.depth_view = depth_view;
    }

    /// Draws the renderer, camera, and config state to the window.
    pub fn draw(
        &mut self,
        camera: &Camera,
        config: &DrawConfig,
    ) -> Result<(), DrawError> {
        let aspect_ratio = self.surface_config.width as f64
            / self.surface_config.height as f64;
        let uniforms = Uniforms {
            transform: Transform::for_vertices(camera, aspect_ratio),
            transform_normals: Transform::for_normals(camera),
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let surface_texture = self.surface.get_current_texture()?;
        let color_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        self.clear_views(&mut encoder, &color_view);

        let drawables = Drawables::new(&self.geometries, &self.pipelines);

        if config.draw_model {
            drawables.model.draw(
                &mut encoder,
                &color_view,
                &self.depth_view,
                &self.bind_group,
            );
        }
        if config.draw_mesh {
            drawables.mesh.draw(
                &mut encoder,
                &color_view,
                &self.depth_view,
                &self.bind_group,
            );
        }
        if config.draw_debug {
            drawables.lines.draw(
                &mut encoder,
                &color_view,
                &self.depth_view,
                &self.bind_group,
            );
        }

        self.config_ui
            .draw(
                &self.device,
                &mut encoder,
                &color_view,
                &self.surface_config,
                &self.geometries.aabb,
                config,
            )
            .map_err(DrawError::Text)?;

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));

        debug!("Presenting...");
        surface_texture.present();

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

        texture.create_view(&wgpu::TextureViewDescriptor::default())
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
/// Error describing the set of render surface initialization errors.
pub enum InitError {
    #[error("I/O error: {0}")]
    /// General IO error
    Io(#[from] io::Error),

    #[error("Error request adapter")]
    /// Graphics accelerator acquisition error
    ///
    /// TODO Document when this occurs
    RequestAdapter,

    #[error("Error requesting device: {0}")]
    /// Device request errors
    ///
    /// See: [wgpu::RequestDeviceError](https://docs.rs/wgpu/latest/wgpu/struct.RequestDeviceError.html)
    RequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("Error loading font: {0}")]
    /// Error loading font
    ///
    /// See: [ab_glyph::InvalidFont](https://docs.rs/ab_glyph/latest/ab_glyph/struct.InvalidFont.html)
    InvalidFont(#[from] InvalidFont),
}

#[derive(Error, Debug)]
/// Graphics rendering error.
///
/// Describes errors related to non intialization graphics errors.
pub enum DrawError {
    #[error("Error acquiring output surface: {0}")]
    /// Surface drawing error.
    ///
    /// See - [wgpu::SurfaceError](https://docs.rs/wgpu/latest/wgpu/enum.SurfaceError.html)
    Surface(#[from] wgpu::SurfaceError),

    #[error("Error drawing text: {0}")]
    /// Text rasterisation error.
    Text(String),
}
