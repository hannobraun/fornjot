use std::{collections::BTreeSet, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{geometry::OpsLog, render::Renderer};

pub fn run(mut ops: OpsLog) -> anyhow::Result<()> {
    ops.select_last();

    let event_loop = EventLoop::new()?;

    let mut app = App {
        ops,
        window: None,
        renderer: None,
        pressed_keys: BTreeSet::new(),
    };
    event_loop.run_app(&mut app)?;

    Ok(())
}

struct App {
    ops: OpsLog,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    pressed_keys: BTreeSet<Key>,
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
        let (Some(window), Some(renderer)) =
            (self.window.as_ref(), self.renderer.as_mut())
        else {
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
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key, state, ..
                    },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        if self.pressed_keys.contains(&logical_key) {
                            return;
                        }
                    }
                    ElementState::Released => {
                        self.pressed_keys.remove(&logical_key);
                        return;
                    }
                }

                match logical_key {
                    Key::Named(NamedKey::ArrowDown) => {
                        self.ops.select_next();
                    }
                    Key::Named(NamedKey::ArrowUp) => {
                        self.ops.select_previous();
                    }
                    _ => {}
                }

                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Some(selected) =
                    self.ops.operations.get(self.ops.selected)
                {
                    if let Err(err) = renderer.render(selected, &self.ops) {
                        eprintln!("Render error: {err}");
                    }
                }
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
