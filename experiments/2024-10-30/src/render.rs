use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::mesh::Mesh;

pub fn render(_: &Mesh) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            },
        ))
        .unwrap();
        let (device, queue) = pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        )
        .unwrap();

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        self.window = Some(window);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        let Some(device) = self.device.as_ref() else {
            return;
        };
        let Some(queue) = self.queue.as_ref() else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                let frame = surface.get_current_texture().unwrap();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor::default(),
                );

                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    ..Default::default()
                });

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    }
}
