//! Types that describe aspects of the screen

pub use raw_window_handle::HasRawWindowHandle;

/// Needs to be implemented by types that can serve as a screen to render to
pub trait Screen {
    /// The window
    type Window: HasRawWindowHandle;

    /// Access the size of the screen
    fn size(&self) -> Size;

    /// Access the window
    fn window(&self) -> &Self::Window;
}

/// A position on the screen
#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// The x coordinate of the position
    pub x: f64,

    /// The y coordinate of the position
    pub y: f64,
}

/// The size of the screen
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The width of the screen
    pub width: u32,

    /// The height of the screen
    pub height: u32,
}

impl Size {
    /// Convert size to `f64`
    pub fn as_f64(&self) -> [f64; 2] {
        [self.width, self.height].map(Into::into)
    }
}
