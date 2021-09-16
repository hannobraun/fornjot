use nalgebra::{vector, Point, SVector};

use crate::geometry::aabb::Aabb;

/// Defines geometry that can be sampled
///
/// The `D` parameter defines the dimensionality of the geometry (typically
/// geometry would be 2- or 3-dimensional).
pub trait Geometry<const D: usize> {
    /// Sample the geometry at the specified point
    ///
    /// Returns a `Sample` value which describes, among other attributes, the
    /// distance of the point from the surface.
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Distance<D>;
}

/// The minimum distance of a specific point to a surface
///
/// Returned by [`Geometry::sample`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<const D: usize> {
    /// The point from which the distance was determined
    pub point: Point<f32, D>,

    /// The minimum distance of the point to the surface
    ///
    /// A positive value indicates that the point is outside of the object, a
    /// negative value indicates that the point is inside. Either way, the
    /// absolute value is equal to the distance of the point to the surface.
    pub distance: f32,
}

/// Implemented for geometry that can return surface normals
///
/// The `D` parameter defines the dimensionality of the geometry. Blanked
/// implementations for 2- and 3-dimensional geometry (i.e. implementations of
/// `Geometry<2>` and `Geometry<3>`) exist.
pub trait Normal<const D: usize> {
    /// Return the surface normal at the given point
    fn normal(&self, point: impl Into<Point<f32, D>>) -> SVector<f32, D>;
}

impl<T> Normal<2> for T
where
    T: Geometry<2>,
{
    fn normal(&self, point: impl Into<Point<f32, 2>>) -> SVector<f32, 2> {
        const EPSILON: f32 = 0.1;

        let point = point.into();

        let eps_x = vector![EPSILON, 0.0];
        let eps_y = vector![0.0, EPSILON];

        let dir = vector![
            self.sample(point + eps_x).distance
                - self.sample(point - eps_x).distance,
            self.sample(point + eps_y).distance
                - self.sample(point - eps_y).distance
        ];

        dir.normalize()
    }
}

impl<T> Normal<3> for T
where
    T: Geometry<3>,
{
    fn normal(&self, point: impl Into<Point<f32, 3>>) -> SVector<f32, 3> {
        const EPSILON: f32 = 0.1;

        let point = point.into();

        let eps_x = vector![EPSILON, 0.0, 0.0];
        let eps_y = vector![0.0, EPSILON, 0.0];
        let eps_z = vector![0.0, 0.0, EPSILON];

        let dir = vector![
            self.sample(point + eps_x).distance
                - self.sample(point - eps_x).distance,
            self.sample(point + eps_y).distance
                - self.sample(point - eps_y).distance,
            self.sample(point + eps_z).distance
                - self.sample(point - eps_z).distance
        ];

        dir.normalize()
    }
}

/// Defines a bounding volume that encloses geometry
pub trait BoundingVolume<const D: usize> {
    /// Return the geometry's axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use crate::geometry::shapes::{Circle, Sphere};

    use super::Normal as _;

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
