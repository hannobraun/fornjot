use std::{f32::consts::PI, sync::Arc};

use anyhow::anyhow;
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::geometry::Operation;

use super::{
    geometry::Geometry,
    pipeline::Pipeline,
    shaders::{Shaders, Uniforms},
};

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: Pipeline,
    pub depth_view: wgpu::TextureView,
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

        let aspect_ratio = size.width as f32 / size.height as f32;
        let uniforms =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[Uniforms::from_transform(
                    default_transform(aspect_ratio),
                )]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let triangles_shaders = Shaders::triangles(&device, &config);
        let pipeline = Pipeline::new(&device, &triangles_shaders, &uniforms);

        let depth_view = {
            let depth_texture =
                device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: config.width,
                        height: config.height,
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

        Ok(Self {
            surface,
            device,
            queue,
            pipeline,
            depth_view,
        })
    }

    pub fn render(&self, operation: &impl Operation) {
        let vertices = Geometry::vertices(&self.device, operation);
        let triangles = Geometry::triangles(&self.device, operation);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let frame = self.surface.get_current_texture().unwrap();
        let color_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            // No need to do anything else with the render pass. Creating and
            // then dropping it is enough to clear the background.
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
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
        }

        self.pipeline.draw(
            &mut encoder,
            &color_view,
            &self.depth_view,
            &vertices,
        );
        self.pipeline.draw(
            &mut encoder,
            &color_view,
            &self.depth_view,
            &triangles,
        );

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

fn default_transform(aspect_ratio: f32) -> Mat4 {
    let fov_y_radians = std::f32::consts::PI / 2.;
    let z_near = 0.1;
    let z_far = 10.;

    Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)
        * Mat4::from_translation(Vec3::new(0., 0., -2.))
        * Mat4::from_rotation_x(-PI / 4.)
        * Mat4::from_rotation_z(PI / 4.)
}
