use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{mesh::Mesh, render::Renderer};

pub fn run(_: Mesh) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, renderer) = match init(event_loop) {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("Initialization error: `{err:?}`");
                event_loop.exit();
                return;
            }
        };

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(renderer) = self.renderer.as_ref() else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                let frame = renderer.surface.get_current_texture().unwrap();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = renderer.device.create_command_encoder(
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

                renderer.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    }
}

fn init(
    event_loop: &ActiveEventLoop,
) -> anyhow::Result<(Arc<Window>, Renderer)> {
    let window = {
        let window = event_loop.create_window(WindowAttributes::default())?;
        Arc::new(window)
    };
    let renderer = pollster::block_on(Renderer::new(window.clone()))?;

    Ok((window, renderer))
}
