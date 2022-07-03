use fj_math::{Transform, Vector};

use crate::camera::Camera;

pub struct Zoom {
    accumulated_delta: f64,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            accumulated_delta: 0.0,
        }
    }

    pub fn push(&mut self, delta: f64) {
        // Accumulate all zoom inputs
        self.accumulated_delta += delta;
    }

    pub fn apply_to_camera(&mut self, delta_t: f64, camera: &mut Camera) {
        let displacement = self.accumulated_delta
            * delta_t
            * ZOOM_FACTOR
            * camera.position().coords.magnitude().into_f64();
        camera.translation = camera.translation
            * Transform::translation(Vector::from([0.0, 0.0, -displacement]));

        self.accumulated_delta = 0.;
    }
}

/// Affects the speed of zoom movement.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR: f64 = 0.05;
