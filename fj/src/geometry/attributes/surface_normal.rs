use nalgebra::{Point, SVector, vector};

use super::SignedDistanceField;

/// Implemented for geometry that can return surface normals
///
/// The `D` parameter defines the dimensionality of the geometry. Blanked
/// implementations for 2- and 3-dimensional geometry (i.e. implementations of
/// `Geometry<2>` and `Geometry<3>`) exist.
pub trait SurfaceNormal<const D: usize> {
    /// Return the surface normal at the given point
    fn normal(&self, point: impl Into<Point<f32, D>>) -> SVector<f32, D>;
}

impl<T> SurfaceNormal<2> for T
where
    T: SignedDistanceField<2>,
{
    fn normal(&self, point: impl Into<Point<f32, 2>>) -> SVector<f32, 2> {
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
    fn normal(&self, point: impl Into<Point<f32, 3>>) -> SVector<f32, 3> {
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
