use std::{io, mem::size_of, vec};

use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;

use crate::{
    camera::Camera,
    gui::Gui,
    screen::{Screen, ScreenSize},
};

use super::{
    draw_config::DrawConfig, drawables::Drawables, geometries::Geometries,
    navigation_cube::NavigationCubeRenderer, pipelines::Pipelines,
    transform::Transform, uniforms::Uniforms, vertices::Vertices, DEPTH_FORMAT,
    SAMPLE_COUNT,
};

/// Graphics rendering state and target abstraction
#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    features: wgpu::Features,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,
    frame_buffer: wgpu::TextureView,
    depth_view: wgpu::TextureView,

    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    geometries: Geometries,
    pipelines: Pipelines,

    navigation_cube_renderer: NavigationCubeRenderer,
}

impl Renderer {
    /// Returns a new `Renderer`.
    pub async fn new(screen: &impl Screen) -> Result<Self, RendererInitError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // This is sound, as `window` is an object to create a surface upon.
        let surface = unsafe { instance.create_surface(screen.window()) }?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(RendererInitError::RequestAdapter)?;

        let features = {
            let desired_features = wgpu::Features::POLYGON_MODE_LINE;
            let available_features = adapter.features();

            // By requesting the intersection of desired and available features,
            // we prevent two things:
            //
            // 1. That requesting the device panics, which would happen if we
            //    requested unavailable features.
            // 2. That a developer ends up accidentally using features that
            //    happen to be available on their machine, but that aren't
            //    necessarily available for all the users.
            desired_features.intersection(available_features)
        };

        let limits = {
            // This is the lowest of the available defaults. It should guarantee
            // that we can run pretty much everywhere.
            let lowest_limits = wgpu::Limits::downlevel_webgl2_defaults();

            // However, these lowest limits aren't necessarily capable of
            // supporting the screen resolution of our current platform, so
            // let's amend them.
            let supported_limits = adapter.limits();
            lowest_limits.using_resolution(supported_limits)
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                None,
            )
            .await?;

        let color_format = 'color_format: {
            let capabilities = surface.get_capabilities(&adapter);
            let supported_formats = capabilities.formats;

            // We don't really care which color format we use, as long as we
            // find one that's supported. `egui_wgpu` prints a warning though,
            // unless we choose one of the following ones.
            let preferred_formats = [
                wgpu::TextureFormat::Rgba8Unorm,
                wgpu::TextureFormat::Bgra8Unorm,
            ];

            for format in preferred_formats {
                if supported_formats.contains(&format) {
                    break 'color_format format;
                }
            }

            // None of the preferred color formats are supported. Just use one
            // of the supported ones then.
            supported_formats
                .into_iter()
                .next()
                .expect("No color formats supported")
        };

        let ScreenSize { width, height } = screen.size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: color_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            // I don't understand what this option does. It was introduced with
            // wgpu 0.14, but we had already been using premultiplied alpha
            // blending before that. See the `BlendState` configuration of the
            // render pipelines.
            //
            // For that reason, I tried to set this to `PreMultiplied`, but that
            // failed on Linux/Wayland (with in integrated AMD GPU). Setting it
            // to `Auto` seems to just work.
            //
            // @hannobraun
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let frame_buffer = Self::create_frame_buffer(&device, &surface_config);
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

        let geometries =
            Geometries::new(&device, &Vertices::empty(), &Vertices::empty());
        let pipelines =
            Pipelines::new(&device, &bind_group_layout, color_format);

        let navigation_cube_renderer =
            NavigationCubeRenderer::new(&device, &queue, &surface_config);

        Ok(Self {
            surface,
            features,
            device,
            queue,

            surface_config,
            frame_buffer,
            depth_view,

            uniform_buffer,
            bind_group,

            geometries,
            pipelines,

            navigation_cube_renderer,
        })
    }

    pub(crate) fn init_gui(&self) -> Gui {
        Gui::new(&self.device, self.surface_config.format)
    }

    /// Updates the geometry of the model being rendered.
    pub fn update_geometry(&mut self, mesh: Vertices, lines: Vertices) {
        self.geometries = Geometries::new(&self.device, &mesh, &lines);
    }

    /// Resizes the render surface.
    ///
    /// # Arguments
    /// - `size`: The target size for the render surface.
    pub fn handle_resize(&mut self, size: ScreenSize) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);

        self.frame_buffer =
            Self::create_frame_buffer(&self.device, &self.surface_config);
        self.depth_view =
            Self::create_depth_buffer(&self.device, &self.surface_config);
    }

    /// Draws the renderer, camera, and config state to the window.
    pub fn draw(
        &mut self,
        camera: &Camera,
        config: &DrawConfig,
        scale_factor: f32,
        gui: &mut Gui,
    ) -> Result<(), DrawError> {
        let aspect_ratio = f64::from(self.surface_config.width)
            / f64::from(self.surface_config.height);
        let uniforms = Uniforms {
            transform: Transform::for_vertices(camera, aspect_ratio),
            transform_normals: Transform::for_normals(camera),
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(wgpu::SurfaceError::Timeout) => {
                // I'm seeing this all the time now (as in, multiple times per
                // microsecond), with `PresentMode::AutoVsync`. Not sure what's
                // going on, but for now, it works to just ignore it.
                //
                // Issues for reference:
                // - https://github.com/gfx-rs/wgpu/issues/1218
                // - https://github.com/gfx-rs/wgpu/issues/1565
                return Ok(());
            }
            result => result?,
        };
        let color_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [
                self.surface_config.width,
                self.surface_config.height,
            ],
            pixels_per_point: scale_factor,
        };
        let clipped_primitives = gui.prepare_draw(
            &self.device,
            &self.queue,
            &mut encoder,
            &screen_descriptor,
        );

        // Need this block here, as a render pass only takes effect once it's
        // dropped.
        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &self.frame_buffer,
                            resolve_target: Some(&color_view),
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                // Not necessary, due to MSAA being enabled.
                                store: false,
                            },
                        },
                    )],
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
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            let drawables = Drawables::new(&self.geometries, &self.pipelines);

            if config.draw_model {
                drawables.model.draw(&mut render_pass);
            }

            if self.is_line_drawing_available() {
                if config.draw_mesh {
                    drawables.mesh.draw(&mut render_pass);
                }
                if config.draw_debug {
                    drawables.lines.draw(&mut render_pass);
                }
            }

            gui.draw(&mut render_pass, &clipped_primitives, &screen_descriptor);
        }

        self.navigation_cube_renderer.draw(
            &color_view,
            &mut encoder,
            &self.queue,
            aspect_ratio,
            camera.rotation,
        );

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));

        debug!("Presenting...");
        surface_texture.present();

        debug!("Finished drawing.");
        Ok(())
    }

    fn create_frame_buffer(
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
            sample_count: SAMPLE_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: surface_config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
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
            sample_count: SAMPLE_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Returns true if the renderer's adapter can draw lines
    pub fn is_line_drawing_available(&self) -> bool {
        self.features.contains(wgpu::Features::POLYGON_MODE_LINE)
    }
}

/// Error describing the set of render surface initialization errors
#[derive(Error, Debug)]
pub enum RendererInitError {
    /// General IO error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Surface creating error
    #[error("Error creating surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    /// Graphics accelerator acquisition error
    #[error("Error request adapter")]
    RequestAdapter,

    /// Device request errors
    ///
    /// See: [wgpu::RequestDeviceError](https://docs.rs/wgpu/latest/wgpu/struct.RequestDeviceError.html)
    #[error("Error requesting device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}

/// Graphics rendering error
///
/// Describes errors related to non initialization graphics errors.
#[derive(Error, Debug)]
pub enum DrawError {
    /// Surface drawing error.
    ///
    /// See - [wgpu::SurfaceError](https://docs.rs/wgpu/latest/wgpu/enum.SurfaceError.html)
    #[error("Error acquiring output surface: {0}")]
    Surface(#[from] wgpu::SurfaceError),

    /// Text rasterisation error.
    #[error("Error drawing text: {0}")]
    Text(String),
}
