use nalgebra::Rotation3;

use crate::{graphics::Camera, math::Vector};

pub struct Rotation;

impl Rotation {
    pub fn apply(&self, angle_x: f64, angle_y: f64, camera: &mut Camera) {
        let rot_x = Rotation3::from_axis_angle(&Vector::x_axis(), angle_x);
        let rot_y = Rotation3::from_axis_angle(&Vector::y_axis(), angle_y);

        camera.rotation = rot_y * rot_x * camera.rotation;
    }
}
