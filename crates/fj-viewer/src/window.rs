//! CAD viewer utility windowing abstraction

use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::screen::{Screen, Size};

/// Window abstraction providing details such as the width or height and easing initialization.
pub struct Window(winit::window::Window);

impl Window {
    /// Returns a new window with the given `EventLoop`.
    ///
    /// # Examples
    /// ```rust no_run
    /// let event_loop = winit::event_loop::EventLoop::new();
    /// let window = fj_viewer::window::Window::new(&event_loop);
    /// ```
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

    fn size(&self) -> Size {
        let size = self.0.inner_size();

        Size {
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
