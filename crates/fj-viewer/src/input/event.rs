use crate::screen::NormalizedPosition;

/// An input event
pub enum Event {
    /// Move the model up, down, left or right
    Translate {
        /// The normalized position of the cursor before input
        previous: NormalizedPosition,
        /// The normalized position of the cursor after input
        current: NormalizedPosition,
    },

    /// Rotate the model around the focus point
    Rotation {
        /// The normalized position of the cursor before input
        previous: NormalizedPosition,
        /// The normalized position of the cursor after input
        current: NormalizedPosition,
    },

    /// Move the view forwards and backwards
    Zoom(f64),
}
