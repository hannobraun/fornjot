use crate::screen::Position;

/// An input event
pub enum Event {
    /// The cursor has moved to another position
    CursorMoved(Position),

    /// A key has been pressed or released
    Key(Key, KeyState),

    /// The user scrolled
    Scroll(MouseScrollDelta),
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
