//! Types that describe aspects of the screen

/// A position on the screen
#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// The x coordinate of the position
    pub x: f64,

    /// The y coordinate of the position
    pub y: f64,
}
