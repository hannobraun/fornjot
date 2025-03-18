use std::sync::Arc;

use anyhow::anyhow;
use winit::window::Window;

use crate::view::OperationView;

use super::{geometry::Geometry, pipeline::Pipeline, text::TextRenderer};

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub pipeline: Pipeline,
    pub depth_view: wgpu::TextureView,
    pub text_renderer: TextRenderer,
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
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| anyhow!("Failed to get default surface config"))?;
        surface.configure(&device, &surface_config);

        let pipeline = Pipeline::new(&device, &surface_config);

        let depth_view = {
            let depth_texture =
                device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: surface_config.width,
                        height: surface_config.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth32Float,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                });

            depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
        };

        let text_renderer = TextRenderer::new(
            &device,
            &queue,
            &surface_config,
            window.scale_factor() as f32,
        );

        Ok(Self {
            surface,
            device,
            queue,
            surface_config,
            pipeline,
            depth_view,
            text_renderer,
        })
    }

    pub fn render(&mut self, operations: &OperationView) -> anyhow::Result<()> {
        let selected_operation = operations.selected();

        let geometry = Geometry::new(&self.device, selected_operation);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let frame = self.surface.get_current_texture().unwrap();
        let color_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &color_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: &self.depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        },
                    ),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            self.pipeline.draw(&mut render_pass, &geometry);
            self.text_renderer.render(
                operations,
                &self.device,
                &self.queue,
                &self.surface_config,
                &mut render_pass,
            )?;
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
