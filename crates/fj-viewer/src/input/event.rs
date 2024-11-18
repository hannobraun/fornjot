/// An input event
pub enum InputEvent {
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
