use crate::screen::Position;

/// An input event
pub enum Event {
    /// The cursor has moved to another position
    CursorMoved(Position),

    /// A key has been pressed
    KeyPressed(Key),
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
}
