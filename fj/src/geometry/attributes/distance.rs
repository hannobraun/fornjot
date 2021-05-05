use nalgebra::Point;

use crate::geometry::{operations, shapes};

/// Provides a signed distance function
pub trait Distance {
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32;
}

impl Distance for shapes::Cylinder {
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

impl<A, B> Distance for operations::Difference<A, B>
where
    A: Distance,
    B: Distance,
{
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32 {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        f32::max(dist_a, -dist_b)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::Cylinder;

    use super::Distance as _;

    #[test]
    fn cylinder_should_return_distance() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        assert_eq!(cylinder.distance([0.0, 0.0, 0.0]), -0.5);
        assert_eq!(cylinder.distance([0.25, 0.0, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.25, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.0, 0.25]), -0.25);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -1.0]), 0.5);
        assert_eq!(cylinder.distance([1.0, 0.0, 2.0]), 0.5);
    }
}
