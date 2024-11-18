//! User input parsing and propagation.

mod event;

pub use self::event::InputEvent;

/// # Any mouse button that is relevant to the operation of this crate
#[derive(Eq, PartialEq)]
pub enum MouseButton {
    /// # The left mouse button
    Left,

    /// # The right mouse button
    Right,
}
