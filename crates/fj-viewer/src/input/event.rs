use crate::screen::NormalizedScreenPosition;

/// An input event
pub enum InputEvent {
    /// Move the model up, down, left or right
    Translation {
        /// The normalized position of the cursor before input
        previous: NormalizedScreenPosition,

        /// The normalized position of the cursor after input
        current: NormalizedScreenPosition,
    },

    /// Rotate the model around the focus point
    Rotation {
        /// The angle around the screen x axis to rotate (in radians)
        angle_x: f64,

        /// The angle around the screen y axis to rotate (in radians)
        angle_y: f64,
    },

    /// Move the view forwards and backwards
    Zoom(f64),
}
