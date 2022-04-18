//! Cad viewer utility windowing abstraction

use winit::{event_loop::EventLoop, window::WindowBuilder};

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
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Fornjot")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)
            .unwrap();

        Self(window)
    }

    /// Returns a shared reference to the wrapped window
    pub fn inner(&self) -> &winit::window::Window {
        &self.0
    }

    /// Returns the width of the window
    pub fn width(&self) -> u32 {
        self.0.inner_size().width
    }

    /// Returns the height of the window
    pub fn height(&self) -> u32 {
        self.0.inner_size().height
    }
}
