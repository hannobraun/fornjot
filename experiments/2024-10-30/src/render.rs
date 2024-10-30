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

    let mut app = App { window: None };
    event_loop.run_app(&mut app)?;

    Ok(())
}

struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent,
    ) {
    }
}
