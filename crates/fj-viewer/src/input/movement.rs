use nalgebra::distance;
use winit::dpi::PhysicalPosition;

use crate::{
    camera::{Camera, FocusPoint},
    window::Window,
};

pub struct Movement {
    focus_point: FocusPoint,
    cursor: Option<PhysicalPosition<f64>>,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            focus_point: FocusPoint::none(),
            cursor: None,
        }
    }

    pub fn start(
        &mut self,
        focus_point: FocusPoint,
        cursor: Option<PhysicalPosition<f64>>,
    ) {
        self.focus_point = focus_point;
        self.cursor = cursor;
    }

    pub fn stop(&mut self) {
        self.focus_point = FocusPoint::none();
    }

    pub fn apply(
        &mut self,
        cursor: Option<PhysicalPosition<f64>>,
        camera: &mut Camera,
        window: &Window<winit::window::Window>,
    ) {
        if let (Some(previous), Some(cursor)) = (self.cursor, cursor) {
            let previous = camera.cursor_to_model_space(previous, window);
            let cursor = camera.cursor_to_model_space(cursor, window);

            if let Some(focus_point) = self.focus_point.0 {
                let d1 = distance(&camera.position(), &cursor);
                let d2 = distance(&camera.position(), &focus_point);

                let diff = (cursor - previous) * d2 / d1;
                let offset = camera.camera_to_model().transform_vector(&diff);

                camera.translation.x += offset.x;
                camera.translation.y += offset.y;
            }
        }

        self.cursor = cursor;
    }
}
