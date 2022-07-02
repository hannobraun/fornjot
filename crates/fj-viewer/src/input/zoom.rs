use std::time::Instant;

use fj_math::{Transform, Vector};

use crate::camera::Camera;

pub struct Zoom {
    current_speed: f64,
}

impl Zoom {
    pub fn new() -> Self {
        Self { current_speed: 0.0 }
    }

    pub fn push(&mut self, delta: f64) {
        // Accumulate all zoom inputs
        self.current_speed += delta * ACCELERATION;
    }

    pub fn apply_to_camera(&mut self, delta_t: f64, camera: &mut Camera) {
        let distance: f64 = camera.position().coords.magnitude().into();
        let displacement = self.current_speed * delta_t * distance;
        camera.translation = camera.translation
            * Transform::translation(Vector::from([0.0, 0.0, -displacement]));

        self.current_speed = 0.;
    }
}

/// Acceleration value for the zoom movement
///
/// Tuning notes:
/// - If this value is too low, target zoom speed will be reached slowly,
///   leading to less precise control.
/// - If this value is too high, zoom movement seems unnatural, which can cause
///   a jarring experience.
///
/// This value should be as high as possible, while not causing jarring
/// accelerations.
const ACCELERATION: f64 = 1.;
