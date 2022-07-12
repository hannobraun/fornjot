use fj_math::{Transform, Vector};

use crate::camera::{Camera, FocusPoint};

pub struct Zoom;

impl Zoom {
    pub fn apply_to_camera(
        &mut self,
        zoom_delta: f64,
        focus_point: &FocusPoint,
        camera: &mut Camera,
    ) {
        let distance = match focus_point.0 {
            Some(fp) => (fp - camera.position()).magnitude(),
            None => camera.position().coords.magnitude(),
        };
        let displacement = zoom_delta * distance.into_f64();
        camera.translation = camera.translation
            * Transform::translation(Vector::from([0.0, 0.0, -displacement]));
    }
}
