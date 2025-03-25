use std::sync::Arc;

/// A window that can be used with `fj-viewer`
pub struct Window {
    pub inner: Arc<winit::window::Window>,
}

impl Window {
    pub fn size(&self) -> WindowSize {
        let size = self.inner.inner_size();

        WindowSize {
            width: size.width,
            height: size.height,
        }
    }

    pub fn winit_window(&self) -> Arc<winit::window::Window> {
        self.inner.clone()
    }
}

/// Cursor position in normalized coordinates (-1 to +1)
///
/// The center of the screen is at (0, 0). The aspect ratio is taken into
/// account.
#[derive(Clone, Copy, Debug)]
pub struct NormalizedScreenPosition {
    /// The x coordinate of the position [-1, 1]
    pub x: f64,

    /// The y coordinate of the position [-1, 1]
    pub y: f64,
}

/// The size of the screen
#[derive(Clone, Copy, Debug)]
pub struct WindowSize {
    /// The width of the screen
    pub width: u32,

    /// The height of the screen
    pub height: u32,
}

impl WindowSize {
    /// # Indicate whether the screen size is valid
    ///
    /// A screen size is valid, if neither of its dimensions is zero. But it can
    /// be reported as zero by spurious screen resize events.
    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Convert size to `f64`
    pub fn as_f64(&self) -> [f64; 2] {
        [self.width, self.height].map(Into::into)
    }
}
