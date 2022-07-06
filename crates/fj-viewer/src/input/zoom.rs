use fj_math::{Transform, Vector};

use crate::camera::{Camera, FocusPoint};

use super::event::MouseScrollDelta;

pub struct Zoom {
    accumulated_delta: f64,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            accumulated_delta: 0.0,
        }
    }

    pub fn push(&mut self, delta: MouseScrollDelta) {
        // Accumulate all zoom inputs
        self.accumulated_delta += match delta {
            MouseScrollDelta::Line(delta) => ZOOM_FACTOR_LINE * delta,
            MouseScrollDelta::Pixel(delta) => ZOOM_FACTOR_PIXEL * delta,
        };
    }

    pub fn apply_to_camera(
        &mut self,
        delta_t: f64,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        let distance = match focus_point.0 {
            Some(fp) => (fp - camera.position()).magnitude(),
            None => camera.position().coords.magnitude(),
        };
        let displacement =
            self.accumulated_delta * delta_t * distance.into_f64();
        camera.translation = camera.translation
            * Transform::translation(Vector::from([0.0, 0.0, -displacement]));

        self.accumulated_delta = 0.;
    }
}

/// Affects the speed of zoom movement given a scroll wheel input in lines.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_LINE: f64 = 15.0;

/// Affects the speed of zoom movement given a scroll wheel input in pixels.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_PIXEL: f64 = 1.0;
