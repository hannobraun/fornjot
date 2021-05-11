use nalgebra::Point;

use crate::geometry::{operations, shapes};

/// Provides a signed distance function
pub trait Distance<const D: usize> {
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32;
}

impl Distance<2> for shapes::Circle {
    fn distance(&self, point: impl Into<Point<f32, 2>>) -> f32 {
        let point = point.into();

        point.coords.magnitude() - self.radius
    }
}

impl<A, B, const D: usize> Distance<D> for operations::Difference<A, B>
where
    A: Distance<D>,
    B: Distance<D>,
{
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32 {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        f32::max(dist_a, -dist_b)
    }
}

impl<Sketch> Distance<3> for operations::LinearExtrude<Sketch>
where
    Sketch: Distance<2>,
{
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> f32 {
        let point = point.into();

        let d_xy = self.sketch.distance(point.xy());
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
