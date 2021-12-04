use nalgebra::{Rotation3, Translation};

use crate::{
    camera::Camera,
    math::{Point, Vector},
};

pub struct Rotation;

impl Rotation {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(
        &self,
        center: Point,
        angle_x: f64,
        angle_y: f64,
        camera: &mut Camera,
    ) {
        let trans = Translation::from(center.coords);

        let rot_x = Rotation3::from_axis_angle(&Vector::x_axis(), angle_x);
        let rot_y = Rotation3::from_axis_angle(&Vector::y_axis(), angle_y);

        camera.rotation =
            trans * rot_y * rot_x * trans.inverse() * camera.rotation;
    }
}
