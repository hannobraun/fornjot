//! CAD viewer utility windowing abstraction

use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::screen::Size;

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

    /// Returns a shared reference to the wrapped window
    pub fn inner(&self) -> &winit::window::Window {
        &self.0
    }

    /// Returns the size of the window
    pub fn size(&self) -> Size {
        let size = self.0.inner_size();

        Size {
            width: size.width,
            height: size.height,
        }
    }
}

/// Error initializing window
#[derive(Debug, thiserror::Error)]
#[error("Error initializing window")]
pub struct Error(#[from] pub winit::error::OsError);
