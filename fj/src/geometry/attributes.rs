//! Attributes of geometry
//!
//! Contains traits and supporting types that define various attributes that
//! geometry can have.

pub mod signed_distance_field;
pub mod surface_normal;

pub use self::{
    signed_distance_field::{Distance, SignedDistanceField},
    surface_normal::SurfaceNormal,
};

use crate::geometry::aabb::Aabb;

/// Defines a bounding volume that encloses geometry
pub trait BoundingVolume<const D: usize> {
    /// Return the geometry's axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
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
