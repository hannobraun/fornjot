use nalgebra::distance;
use winit::dpi::PhysicalPosition;

use crate::{camera::Camera, math::Point, window::Window};

pub struct Movement {
    focus_point: Option<Point>,
    cursor: Option<PhysicalPosition<f64>>,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            focus_point: None,
            cursor: None,
        }
    }

    pub fn start(
        &mut self,
        focus_point: Option<Point>,
        cursor: Option<PhysicalPosition<f64>>,
    ) {
        self.focus_point = focus_point;
        self.cursor = cursor;
    }

    pub fn stop(&mut self) {
        self.focus_point = None;
    }

    pub fn apply(
        &mut self,
        cursor: Option<PhysicalPosition<f64>>,
        camera: &mut Camera,
        window: &Window,
    ) {
        if let (Some(previous), Some(cursor)) = (self.cursor, cursor) {
            let previous = camera.cursor_to_model_space(previous, window);
            let cursor = camera.cursor_to_model_space(cursor, window);

            if let Some(focus_point) = self.focus_point {
                let d1 = distance(&camera.position(), &cursor);
                let d2 = distance(&camera.position(), &focus_point);

                let diff = (cursor - previous) * d2 / d1;

                // TASK: This doesn't take rotation into account, and thus
                //       doesn't work correctly if any rotation has been
                //       applied.
                camera.translation.x += diff.x;
                camera.translation.y += diff.y;
            }
        }

        self.cursor = cursor;
    }
}
