use crate::screen::NormalizedPosition;

/// An input event
pub enum Event {
    /// Move the view up, down, left or right
    Pan {
        /// The normalized position of the cursor before input
        previous: NormalizedPosition,
        /// The normalized position of the cursor after input
        current: NormalizedPosition,
    },

    /// Rotate the view around the focus point
    Orbit {
        /// The normalized position of the cursor before input
        previous: NormalizedPosition,
        /// The normalized position of the cursor after input
        current: NormalizedPosition,
    },

    /// Move the view forwards and backwards
    Zoom(f64),
}

/// Describes a difference in the vertical mouse scroll wheel state.
/// Positive values indicate movement forward (away from the user).
pub enum MouseScrollDelta {
    /// Amount in lines to scroll.
    Line(f64),
    /// Amount in pixels to scroll.
    Pixel(f64),
}

/// A keyboard or mouse key
pub enum Key {
    /// The escape key
    Escape,

    /// The numerical key `1`
    Key1,

    /// The numerical key `2`
    Key2,

    /// The numerical key `3`
    Key3,

    /// The left mouse key
    MouseLeft,

    /// The right mouse key
    MouseRight,
}

/// Defines the meaning of a key event
pub enum KeyState {
    /// A key was pressed
    Pressed,

    /// A key was released
    Released,
}
