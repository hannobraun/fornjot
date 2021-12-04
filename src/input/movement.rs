use nalgebra::Translation2;

use crate::{camera::Camera, math::Point};

pub struct Movement {
    focus_point: Option<Point>,
}

impl Movement {
    pub fn new() -> Self {
        Self { focus_point: None }
    }

    pub fn start(&mut self, focus_point: Option<Point>) {
        self.focus_point = focus_point;
    }

    pub fn stop(&mut self) {
        self.focus_point = None;
    }

    pub fn apply(&self, diff_x: f64, diff_y: f64, camera: &mut Camera) {
        if let Some(_) = self.focus_point {
            // TASK: Moving feels good, if you're dragging the model exactly
            //       where your mouse goes. It feels weird, if the mouse cursor
            //       moves faster or slower than the model you're moving.
            //
            //       The following factor achieves this good-feeling move for
            //       relatively small models at the default distance between
            //       camera and model origin. It breaks down when moving the
            //       camera closer or away from the model, which is the far more
            //       common case.
            //
            //       It would be nicer to have a zoom factor that depends on the
            //       distance between camera and model origin, or even the
            //       distance between the camera and the part of the model the
            //       mouse is currently pointing at (or more precisely, the
            //       distance between the camera and a plane that touches the
            //       surface of the model where the mouse is pointing, and whose
            //       normal is parallel to the camera's viewing direction).
            let f = 0.2;

            let trans_x = diff_x * f;
            let trans_y = -diff_y * f;

            let translation = Translation2::new(trans_x, trans_y);

            camera.translation = translation * camera.translation;
        }
    }
}
