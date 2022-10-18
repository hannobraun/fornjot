//! CAD viewer utility windowing abstraction

use fj_viewer::screen::{Screen, ScreenSize};
use winit::{event_loop::EventLoop, window::WindowBuilder};

/// Window abstraction providing details such as the width or height and easing initialization.
pub struct Window(winit::window::Window);

impl Window {
    /// Returns a new window with the given `EventLoop`.
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Error> {
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
