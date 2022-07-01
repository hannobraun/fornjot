use std::{io, mem::size_of};

use fj_math::{Aabb, Point};
use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;
use wgpu_glyph::ab_glyph::InvalidFont;

use crate::{
    camera::Camera,
    screen::{Screen, Size},
};

use super::{
    config_ui::ConfigUi, draw_config::DrawConfig, drawables::Drawables,
    geometries::Geometries, pipelines::Pipelines, transform::Transform,
    uniforms::Uniforms, vertices::Vertices, DEPTH_FORMAT,
};

#[derive(Default)]
struct EguiOptionsState {
    show_trace: bool,
    show_layout_debug_on_hover: bool,
    show_debug_text_example: bool,
    show_original_ui: bool,
    show_settings_ui: bool,
    show_inspection_ui: bool,
}

pub struct EguiState {
    pub winit_state: egui_winit::State,
    pub context: egui::Context,
    rpass: egui_wgpu::renderer::RenderPass,
    options: EguiOptionsState,
}

impl std::fmt::Debug for EguiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EguiState {}")
    }
}

/// Graphics rendering state and target abstraction
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

    config_ui: ConfigUi,

    /// State required for integration with `egui`.
    pub egui: EguiState,
}

impl Renderer {
    /// Returns a new `Renderer`.
    pub async fn new(
        screen: &impl Screen<Window = egui_winit::winit::window::Window>,
      ) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let egui_winit_state = egui_winit::State::new(4096, screen.window());
        let egui_context = egui::Context::default();

        // This is sound, as `window` is an object to create a surface upon.
        let surface = unsafe { instance.create_surface(screen.window()) };

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

        let Size { width, height } = screen.size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: color_format,
            width,
            height,
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

        //
        // Note: We need to hold on to this otherwise (from my memory)
        //       it causes the egui font texture to get dropped after
        //       drawing one frame.
        //
        //       This then results in an `egui_wgpu_backend` error of
        //       `BackendError::Internal` with message:
        //
        //           "Texture 0 used but not live"
        //
        //       See also: <https://github.com/hasenbanck/egui_wgpu_backend/blob/b2d3e7967351690c6425f37cd6d4ffb083a7e8e6/src/lib.rs#L373>
        //
        let egui_rpass = egui_wgpu::renderer::RenderPass::new(
            &device,
            surface_config.format,
            1,
        );

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

            egui: EguiState {
                context: egui_context,
                winit_state: egui_winit_state,
                rpass: egui_rpass,
                options: Default::default(),
            },
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

    /// Resizes the render surface.
    ///
    /// # Arguments
    /// - `size`: The target size for the render surface.
    pub fn handle_resize(&mut self, size: Size) {
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

        let egui_input = egui::RawInput::default(); // TODO: Change this to use actual input.
        self.egui.context.begin_frame(egui_input);

        egui::SidePanel::left("fj-left-panel").show(&self.egui.context, |ui| {
            ui.label("Fornjot");
        });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let egui_output = self.egui.context.end_frame();
        let egui_paint_jobs = self.egui.context.tessellate(egui_output.shapes);

        // TODO: Actually render the UI.

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

/// Error describing the set of render surface initialization errors
#[derive(Error, Debug)]
pub enum InitError {
    #[error("I/O error: {0}")]
    /// General IO error
    Io(#[from] io::Error),

    #[error("Error request adapter")]
    /// Graphics accelerator acquisition error
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

/// Graphics rendering error
///
/// Describes errors related to non initialization graphics errors.
#[derive(Error, Debug)]
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

impl Renderer {
    //
    // Note: `egui` changed how it handles updating textures on
    //       the GPU between v0.17.0 & v0.18.0, this means we can't
    //       use the same approach as original proof-of-concept used.
    //
    //       Unfortunately we can't use the helper function provided
    //       by `egui` here, as it is tightly integrated with `Painter`
    //       which assumes it is handling surface creation itself.
    //
    //       Additionally, subsequent code changes significantly
    //       changed the API but haven't yet been released.
    //
    //       And, to top it all off, the `Painter::paint_and_update_textures()`
    //       as it currently exists doesn't support a transparent
    //       clear color, which we rely on to overlay the UI on the
    //       already rendered model.
    //
    //       So, as an interim measure, this code is a copy of the
    //       texture update code from <https://github.com/emilk/egui/blob/f807a290a422f401939bd38236ece3cf86c8ee70/egui-wgpu/src/winit.rs#L102-L136>.
    //
    //       TODO: Add transparency workaround.
    //
    fn paint_and_update_textures(
        &mut self,
        pixels_per_point: f32,
        clear_color: egui::Rgba,
        clipped_primitives: &[egui::ClippedPrimitive],
        textures_delta: &egui::TexturesDelta,
        output_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        // Upload all resources for the GPU.
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [
                self.surface_config.width,
                self.surface_config.height,
            ],
            pixels_per_point,
        };

        for (id, image_delta) in &textures_delta.set {
            self.egui.rpass.update_texture(
                &self.device,
                &self.queue,
                *id,
                image_delta,
            );
        }
        for id in &textures_delta.free {
            self.egui.rpass.free_texture(id);
        }

        self.egui.rpass.update_buffers(
            &self.device,
            &self.queue,
            clipped_primitives,
            &screen_descriptor,
        );

        // Record all render passes.
        self.egui.rpass.execute(
            encoder,
            &output_view,
            clipped_primitives,
            &screen_descriptor,
            Some(wgpu::Color {
                r: clear_color.r() as f64,
                g: clear_color.g() as f64,
                b: clear_color.b() as f64,
                a: clear_color.a() as f64,
            }),
        );
    }
}
