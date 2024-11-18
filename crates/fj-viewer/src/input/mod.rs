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

/// # Sensitivity of camera zoom, given scroll wheel input in lines
///
/// Given a specific input, smaller values mean that the camera moves less,
/// larger values mean it moves more.
pub const CAMERA_ZOOM_SENSITIVITY_LINE: f64 = 0.075;

/// # Sensitivity of camera zoom, given scroll wheel input in pixels
///
/// Given a specific input, smaller values mean that the camera moves less,
/// larger values mean it moves more.
pub const CAMERA_ZOOM_SENSITIVITY_PIXEL: f64 = 0.005;

/// # Sensitivity of camera rotation
///
/// Given a specific input, smaller values mean that the camera rotates less,
/// larger values mean it rotates more.
pub const CAMERA_ROTATION_SENSITIVITY: f64 = 5.;
