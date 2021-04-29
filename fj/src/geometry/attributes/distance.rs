use nalgebra::Point;

use crate::geometry::shapes::Circle;

/// Provides a signed distance function
pub trait Distance {
    fn distance(&self, point: Point<f32, 3>) -> f32;
}

impl Distance for Circle {
    fn distance(&self, point: Point<f32, 3>) -> f32 {
        let dist_2d = point.coords.xy().magnitude() - self.radius();

        if dist_2d > 0.0 {
            (dist_2d * dist_2d + point.z * point.z).sqrt()
        } else {
            point.z
        }
    }
}
