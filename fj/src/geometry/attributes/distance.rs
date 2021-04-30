use nalgebra::Point;

use crate::geometry::shapes::{Circle, Cylinder};

/// Provides a signed distance function
pub trait Distance {
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32;
}

impl Distance for Circle {
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32 {
        let point = point.into();

        let dist_2d = point.coords.xy().magnitude() - self.radius();

        if dist_2d > 0.0 {
            (dist_2d * dist_2d + point.z * point.z).sqrt()
        } else {
            point.z
        }
    }
}

impl Distance for Cylinder {
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32 {
        let point = point.into();

        let d_xy = point.xy().coords.magnitude() - self.radius;
        let d_z = point.z.abs() - self.height / 2.0;

        if d_xy < 0.0 || d_z < 0.0 {
            f32::max(d_xy, d_z)
        } else {
            f32::min(d_xy, d_z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::Cylinder;

    use super::Distance as _;

    #[test]
    fn cylinder_should_return_distance() {
        let cylinder = Cylinder::new().radius(0.5).height(1.0);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -1.0]), 0.5);
        assert_eq!(cylinder.distance([1.0, 0.0, 2.0]), 0.5);
    }
}
