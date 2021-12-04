use nalgebra::{Rotation3, Unit};

use crate::graphics::Camera;

pub struct Rotation;

impl Rotation {
    pub fn apply(&self, angle_x: f64, angle_y: f64, transform: &mut Camera) {
        let rot_x = Rotation3::from_axis_angle(
            &Unit::new_unchecked([1.0, 0.0, 0.0].into()),
            angle_x,
        );
        let rot_y = Rotation3::from_axis_angle(
            &Unit::new_unchecked([0.0, 1.0, 0.0].into()),
            angle_y,
        );

        transform.rotation = rot_y * rot_x * transform.rotation;
    }
}
