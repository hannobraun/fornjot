use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{geometry::OpsLog, render::Renderer};

pub fn run(ops: OpsLog) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = App {
        ops,
        window: None,
        renderer: None,
    };
    event_loop.run_app(&mut app)?;

    Ok(())
}

struct App {
    ops: OpsLog,
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
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(renderer) = self.renderer.as_ref() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                renderer.render(&self.ops);
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
