use nalgebra::vector;

use crate::math::{Point, Vector};

use super::SignedDistanceField;

/// Implemented for geometry that can return surface normals
///
/// The `D` parameter defines the dimensionality of the geometry. Blanked
/// implementations for 2- and 3-dimensional geometry (i.e. implementations of
/// `Geometry<2>` and `Geometry<3>`) exist.
pub trait SurfaceNormal<const D: usize> {
    /// Return the surface normal at the given point
    fn normal(&self, point: impl Into<Point<D>>) -> Vector<D>;
}

impl<T> SurfaceNormal<2> for T
where
    T: SignedDistanceField<2>,
{
    fn normal(&self, point: impl Into<Point<2>>) -> Vector<2> {
        const EPSILON: f32 = 0.1;

        let point = point.into();

        let eps_x = vector![EPSILON, 0.0];
        let eps_y = vector![0.0, EPSILON];

        let dir = vector![
            self.distance(point + eps_x).distance
                - self.distance(point - eps_x).distance,
            self.distance(point + eps_y).distance
                - self.distance(point - eps_y).distance
        ];

        dir.normalize()
    }
}

impl<T> SurfaceNormal<3> for T
where
    T: SignedDistanceField<3>,
{
    fn normal(&self, point: impl Into<Point<3>>) -> Vector<3> {
        const EPSILON: f32 = 0.1;

        let point = point.into();

        let eps_x = vector![EPSILON, 0.0, 0.0];
        let eps_y = vector![0.0, EPSILON, 0.0];
        let eps_z = vector![0.0, 0.0, EPSILON];

        let dir = vector![
            self.distance(point + eps_x).distance
                - self.distance(point - eps_x).distance,
            self.distance(point + eps_y).distance
                - self.distance(point - eps_y).distance,
            self.distance(point + eps_z).distance
                - self.distance(point - eps_z).distance
        ];

        dir.normalize()
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use crate::geometry::shapes::{Circle, Sphere};

    use super::SurfaceNormal as _;

    #[test]
    fn normal_trait_should_be_implemented_for_2d_geometry() {
        #[rustfmt::skip]
        let expected = [
            (point![-1.0,  0.0], vector![-1.0,  0.0]),
            (point![ 1.0,  0.0], vector![ 1.0,  0.0]),
            (point![ 0.0, -1.0], vector![ 0.0, -1.0]),
            (point![ 0.0,  1.0], vector![ 0.0,  1.0]),
        ];

        let circle = Circle::new();
        for (point, normal) in expected {
            assert_eq!(circle.normal(point), normal);
        }
    }

    #[test]
    fn normal_trait_should_be_implemented_for_3d_geometry() {
        #[rustfmt::skip]
        let expected = [
            (point![-1.0,  0.0,  0.0], vector![-1.0,  0.0,  0.0]),
            (point![ 1.0,  0.0,  0.0], vector![ 1.0,  0.0,  0.0]),
            (point![ 0.0, -1.0,  0.0], vector![ 0.0, -1.0,  0.0]),
            (point![ 0.0,  1.0,  0.0], vector![ 0.0,  1.0,  0.0]),
            (point![ 0.0,  0.0, -1.0], vector![ 0.0,  0.0, -1.0]),
            (point![ 0.0,  0.0,  1.0], vector![ 0.0,  0.0,  1.0]),
        ];

        let sphere = Sphere::new();
        for (point, normal) in expected {
            assert_eq!(sphere.normal(point), normal);
        }
    }
}
