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

/// # Tuning configuration for camera input
pub struct CameraTuningConfig {
    /// # Sensitivity of camera zoom, given scroll wheel input in lines
    ///
    /// Given a specific input, smaller values mean that the camera moves less,
    /// larger values mean it moves more.
    pub zoom_sensitivity_line: f64,

    /// # Sensitivity of camera zoom, given scroll wheel input in pixels
    ///
    /// Given a specific input, smaller values mean that the camera moves less,
    /// larger values mean it moves more.
    pub zoom_sensitivity_pixel: f64,

    /// # Sensitivity of camera rotation
    ///
    /// Given a specific input, smaller values mean that the camera rotates less,
    /// larger values mean it rotates more.
    pub rotation_sensitivity: f64,
}

/// # The default camera tuning configuration
pub const DEFAULT_CAMERA_TUNING_CONFIG: CameraTuningConfig =
    CameraTuningConfig {
        zoom_sensitivity_line: 0.075,
        zoom_sensitivity_pixel: 0.005,
        rotation_sensitivity: 5.,
    };
