//! Types that describe aspects of the screen

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
