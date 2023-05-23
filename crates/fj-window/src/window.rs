use fj_viewer::{Screen, ScreenSize};
use winit::{event_loop::EventLoop, window::WindowBuilder};

/// A window that can be used with `fj-viewer`
pub struct Window(winit::window::Window);

impl Window {
    /// Create an instance of `Window` from the given `EventLoop`
    pub fn new<T>(event_loop: &EventLoop<T>) -> Result<Self, Error> {
        let window = WindowBuilder::new()
            .with_title("Fornjot")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)?;

        Ok(Self(window))
    }
}

impl Screen for Window {
    type Window = winit::window::Window;

    fn size(&self) -> ScreenSize {
        let size = self.0.inner_size();

        ScreenSize {
            width: size.width,
            height: size.height,
        }
    }

    fn window(&self) -> &winit::window::Window {
        &self.0
    }
}

/// Error initializing window
#[derive(Debug, thiserror::Error)]
#[error("Error initializing window")]
pub struct Error(#[from] pub winit::error::OsError);
