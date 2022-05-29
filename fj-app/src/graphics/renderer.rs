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

#[derive(Default)]
struct EguiOptionsState {
    show_trace: bool,
    show_layout_debug_on_hover: bool,
    show_debug_text_example: bool,
}

// Temporarily removed due to egui fields not implementing `Debug`.
/* #[derive(Debug)] */
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

    pub(crate) egui_state: egui_winit::State,
    pub(crate) egui_context: egui::Context,

    egui_rpass: egui_wgpu_backend::RenderPass,

    egui_options: EguiOptionsState,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let egui_state = egui_winit::State::new(4096, window.inner());
        let egui_context = egui::Context::default();

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
        let egui_rpass = egui_wgpu_backend::RenderPass::new(
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

            egui_context,
            egui_state,

            egui_rpass,

            egui_options: Default::default(),
        })
    }

    pub fn update_geometry(
        &mut self,
        mesh: Vertices,
        lines: Vertices,
        aabb: Aabb<3>,
    ) {
        self.geometries = Geometries::new(&self.device, &mesh, &lines, aabb);
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);

        let depth_view =
            Self::create_depth_buffer(&self.device, &self.surface_config);
        self.depth_view = depth_view;
    }

    pub fn draw(
        &mut self,
        camera: &Camera,
        config: &mut DrawConfig,
        window: &winit::window::Window, // TODO: Handle this better?
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

        //

        //
        // This integration is basically the result of locating the
        // `.present()` call in the `egui` example, here:
        //
        //     <https://github.com/hasenbanck/egui_example/blob/ca1262a701daf0b20e097ef627fc301ab63339d9/src/main.rs#L177>
        //
        // and then the equivalent call in `renderer.rs`, here:
        //
        //     <https://github.com/hannobraun/Fornjot/blob/15294c2ca2fa5ac5016bb29853943b28952f2dae/fj-app/src/graphics/renderer.rs#L245>
        //
        // Then working backwards from there to merge the functionality.
        //
        // In addition, the following examples were also referenced:
        //
        //  * "Make the example more like an actual use case #17"
        //    <https://github.com/hasenbanck/egui_example/pull/17/files>
        //    This removes some non-essential code from the example
        //    which helps clarify what's *actually* necessary.
        //
        //  * "Update to 0.17, use official winit backend #18"
        //    <https://github.com/hasenbanck/egui_example/pull/18/files>
        //    This uses a more up-to-date `egui` version which
        //    included some API changes.
        //    It's still not the *latest* `egui` version though.
        //

        let egui_input = self.egui_state.take_egui_input(window);
        self.egui_context.begin_frame(egui_input);

        fn get_bbox_size_text(aabb: &Aabb<3>) -> String {
            /* Render size of model bounding box */
            let bbsize = aabb.size().components;
            let info = format!(
                "Model bounding box size: {:0.1} {:0.1} {:0.1}",
                bbsize[0].into_f32(),
                bbsize[1].into_f32(),
                bbsize[2].into_f32()
            );
            info
        }

        // A simple UI
        egui::Window::new("Fornjot").show(&self.egui_context, |ui| {
            ui.checkbox(&mut config.draw_model, "Render model")
                .on_hover_text_at_pointer("Toggle with 1");
            ui.checkbox(&mut config.draw_mesh, "Render mesh")
                .on_hover_text_at_pointer("Toggle with 2");
            ui.checkbox(&mut config.draw_debug, "Render debug")
                .on_hover_text_at_pointer("Toggle with 3");
            ui.add_space(16.0);
            ui.strong(get_bbox_size_text(&self.geometries.aabb));
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
                        "Show debug text example.{}",
                        if self.egui_options.show_debug_text_example {
                            " (Hover me.)"
                        } else {
                            ""
                        }
                    );

                    if ui
                        .checkbox(
                            &mut self.egui_options.show_debug_text_example,
                            label_text,
                        )
                        .hovered()
                    {
                        if self.egui_options.show_debug_text_example {
                            let hover_pos = ui
                                .input()
                                .pointer
                                .hover_pos()
                                .unwrap_or_default();
                            ui.painter().debug_text(
                                hover_pos,
                                egui::Align2::LEFT_TOP,
                                egui::Color32::DEBUG_COLOR,
                                format!("{:#?}", &config),
                            );
                        }
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Demonstration of the `egui` layout debug functionality.
                //
                ui.group(|ui| {

                    if ui
                        .checkbox(
                            &mut self.egui_options.show_layout_debug_on_hover,
                            "Show layout debug on hover.",
                        )
                        .changed()
                    {
                        ui.ctx().set_debug_on_hover(
                            self.egui_options.show_layout_debug_on_hover,
                        );
                    }

                    ui.scope(|ui| {
                        if self.egui_options.show_trace {
                            egui::trace!(ui, format!("{:?}", &config));
                        }
                    });

                    ui.indent("indent-show-trace", |ui| {
                        ui.set_enabled(
                            self.egui_options.show_layout_debug_on_hover,
                        );

                        ui.checkbox(
                            &mut self.egui_options.show_trace,
                            "Also show egui trace.",
                        );

                    });
                });
            }

            ui.add_space(16.0);

        });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let egui_output = self.egui_context.end_frame();
        let egui_paint_jobs = self.egui_context.tessellate(egui_output.shapes);

        // Upload all resources for the GPU.
        let egui_screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: self.surface_config.width,
            physical_height: self.surface_config.height,
            //
            // Note: `scale_factor` can be overridden via `WINIT_X11_SCALE_FACTOR` environment variable,
            //       see: <https://docs.rs/winit/0.26.1/winit/window/struct.Window.html#method.scale_factor>
            //
            scale_factor: window.scale_factor() as f32,
        };

        // Note: For info about the font texture, see:
        //
        //        * <https://docs.rs/egui/0.17.0/egui/enum.TextureId.html#variant.Managed>
        //
        //        * <https://docs.rs/epaint/0.17.0/epaint/textures/struct.TextureManager.html#method.alloc>

        self.egui_rpass
            .add_textures(
                &self.device,
                &self.queue,
                &egui_output.textures_delta,
            )
            .unwrap();

        self.egui_rpass
            .remove_textures(egui_output.textures_delta)
            .unwrap();

        self.egui_rpass.update_buffers(
            &self.device,
            &self.queue,
            &egui_paint_jobs,
            &egui_screen_descriptor,
        );

        tracing::info!("pre-execute (egui_rpass)");

        // Note: There is also an option to use an existing renderpass.
        //
        //       See: <https://docs.rs/egui_wgpu_backend/0.17.0/egui_wgpu_backend/struct.RenderPass.html#method.execute_with_renderpass>

        self.egui_rpass
            .execute(
                &mut encoder,
                &color_view,
                &egui_paint_jobs,
                &egui_screen_descriptor,
                //
                // "Set this to `None` to overlay the UI on top of what's in the framebuffer"
                // via <https://github.com/hasenbanck/egui_example/pull/17/files#diff-42cb6807ad74b3e201c5a7ca98b911c5fa08380e942be6e4ac5807f8377f87fcR132>
                //
                // Alternatively, for initial testing, you can use a colour without alpha
                // (e.g. `Some(wgpu::Color {r:0.5, g:0.0, b:0.0, a:1.0})` ) in order
                // to verify that the renderpass is doing *something*.
                //
                None,
            )
            .unwrap();

        //

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
pub enum InitError {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Error request adapter")]
    RequestAdapter,

    #[error("Error requesting device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("Error loading font")]
    InvalidFont(#[from] InvalidFont),
}

#[derive(Error, Debug)]
pub enum DrawError {
    #[error("Error acquiring output surface")]
    Surface(#[from] wgpu::SurfaceError),

    #[error("Error drawing text")]
    Text(String),
}
