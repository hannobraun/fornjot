use std::{io, mem::size_of};

use fj_interop::status_report::StatusReport;
use fj_math::{Aabb, Point};
use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;
use wgpu_glyph::ab_glyph::InvalidFont;

use crate::{
    camera::Camera,
    gui::Gui,
    screen::{Screen, Size},
};

use super::{
    draw_config::DrawConfig, drawables::Drawables, geometries::Geometries,
    pipelines::Pipelines, transform::Transform, uniforms::Uniforms,
    vertices::Vertices, DEPTH_FORMAT,
};

/// Graphics rendering state and target abstraction
#[derive(Debug)]
pub struct Renderer {
    surface: wgpu::Surface,
    features: wgpu::Features,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,
    depth_view: wgpu::TextureView,

    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    geometries: Geometries,
    pipelines: Pipelines,

    /// State required for integration with `egui`.
    pub gui: Gui,
}

impl Renderer {
    /// Returns a new `Renderer`.
    pub async fn new(screen: &impl Screen) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

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

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let color_format = surface
            .get_supported_formats(&adapter)
            .get(0)
            .copied()
            .expect("Error determining preferred color format");

        let Size { width, height } = screen.size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: color_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
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

        let gui = Gui::new(&device, surface_config.format);

        Ok(Self {
            surface,
            features,
            device,
            queue,

            surface_config,
            depth_view,

            uniform_buffer,
            bind_group,

            geometries,
            pipelines,

            gui,
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
        config: &mut DrawConfig,
        scale_factor: f32,
        status: &mut StatusReport,
        egui_input: egui::RawInput,
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

        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(wgpu::SurfaceError::Timeout) => {
                // I'm seeing this all the time now (as in, multiple times per
                // microsecond), which `PresentMode::AutoVsync`. Not sure what's
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

        if self.is_line_drawing_available() {
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
        }

        self.gui.context.begin_frame(egui_input);

        fn get_bbox_size_text(aabb: &Aabb<3>) -> String {
            /* Render size of model bounding box */
            let bbsize = aabb.size().components;
            let info = format!(
                "Model bounding box size:\n{:0.1} {:0.1} {:0.1}",
                bbsize[0].into_f32(),
                bbsize[1].into_f32(),
                bbsize[2].into_f32()
            );
            info
        }

        let line_drawing_available = self.is_line_drawing_available();

        egui::SidePanel::left("fj-left-panel").show(&self.gui.context, |ui| {
            ui.add_space(16.0);

            ui.group(|ui| {
                ui.checkbox(&mut config.draw_model, "Render model")
                    .on_hover_text_at_pointer("Toggle with 1");
                ui.add_enabled(line_drawing_available, egui::Checkbox::new(&mut config.draw_mesh, "Render mesh"))
                    .on_hover_text_at_pointer("Toggle with 2")
                    .on_disabled_hover_text(
                        "Rendering device does not have line rendering feature support",
                    );
                ui.add_enabled(line_drawing_available, egui::Checkbox::new(&mut config.draw_debug, "Render debug"))
                    .on_hover_text_at_pointer("Toggle with 3")
                    .on_disabled_hover_text(
                        "Rendering device does not have line rendering feature support"
                    );
                ui.add_space(16.0);
                ui.strong(get_bbox_size_text(&self.geometries.aabb));
            });

            ui.add_space(16.0);

            {
                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.gui.options.show_settings_ui,
                        "Show egui settings UI",
                    );
                    if self.gui.options.show_settings_ui {
                        self.gui.context.settings_ui(ui);
                    }
                });

                ui.add_space(16.0);

                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.gui.options.show_inspection_ui,
                        "Show egui inspection UI",
                    );
                    if self.gui.options.show_inspection_ui {
                        ui.indent("indent-inspection-ui", |ui| {
                            self.gui.context.inspection_ui(ui);
                        });
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Originally this was only meant to be a simple demonstration
                // of the `egui` `trace!()` macro...
                //
                // ...but it seems the trace feature can't be enabled
                // separately from the layout debug feature, which all
                // gets a bit messy...
                //
                // ...so, this instead shows one possible way to implement
                // "trace only" style debug text on hover.
                //
                ui.group(|ui| {
                    let label_text = format!(
                        "Show debug text demo.{}",
                        if self.gui.options.show_debug_text_example {
                            " (Hover me.)"
                        } else {
                            ""
                        }
                    );

                    ui.style_mut().wrap = Some(false);

                    if ui
                        .checkbox(
                            &mut self.gui.options.show_debug_text_example,
                            label_text,
                        )
                        .hovered()
                        && self.gui.options.show_debug_text_example
                    {
                        let hover_pos =
                            ui.input().pointer.hover_pos().unwrap_or_default();
                        ui.painter().debug_text(
                            hover_pos,
                            egui::Align2::LEFT_TOP,
                            egui::Color32::DEBUG_COLOR,
                            format!("{:#?}", &config),
                        );
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Demonstration of the `egui` layout debug functionality.
                //
                ui.group(|ui| {
                    //

                    if ui
                        .checkbox(
                            &mut self.gui.options.show_layout_debug_on_hover,
                            "Show layout debug on hover.",
                        )
                        .changed()
                    {
                        ui.ctx().set_debug_on_hover(
                            self.gui.options.show_layout_debug_on_hover,
                        );
                    }

                    ui.scope(|ui| {
                        if self.gui.options.show_trace {
                            egui::trace!(ui, format!("{:?}", &config));
                        }
                    });

                    ui.indent("indent-show-trace", |ui| {
                        ui.set_enabled(
                            self.gui.options.show_layout_debug_on_hover,
                        );

                        ui.checkbox(
                            &mut self.gui.options.show_trace,
                            "Also show egui trace.",
                        );

                        //
                    });
                });
            }

            ui.add_space(16.0);
        });

        egui::Area::new("fj-status-message").show(&self.gui.context, |ui| {
            ui.group(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(format!("Status:{}", status.status()))
                        .color(egui::Color32::BLACK),
                ))
            })
        });

        self.gui.draw(
            &self.device,
            &self.queue,
            &mut encoder,
            &color_view,
            egui_wgpu::renderer::ScreenDescriptor {
                size_in_pixels: [
                    self.surface_config.width,
                    self.surface_config.height,
                ],
                pixels_per_point: scale_factor,
            },
        );

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
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: true,
                },
            })],
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

    /// Returns true if the renderer's adapter can draw lines
    pub fn is_line_drawing_available(&self) -> bool {
        self.features.contains(wgpu::Features::POLYGON_MODE_LINE)
    }
}

/// Error describing the set of render surface initialization errors
#[derive(Error, Debug)]
pub enum InitError {
    /// General IO error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Graphics accelerator acquisition error
    #[error("Error request adapter")]
    RequestAdapter,

    /// Device request errors
    ///
    /// See: [wgpu::RequestDeviceError](https://docs.rs/wgpu/latest/wgpu/struct.RequestDeviceError.html)
    #[error("Error requesting device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),

    /// Error loading font
    ///
    /// See: [ab_glyph::InvalidFont](https://docs.rs/ab_glyph/latest/ab_glyph/struct.InvalidFont.html)
    #[error("Error loading font: {0}")]
    InvalidFont(#[from] InvalidFont),
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
