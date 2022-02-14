pub mod plane;

pub use self::plane::Plane;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

use super::points::SurfacePoint;

/// A two-dimensional shape
#[derive(Clone, Debug, PartialEq)]
pub enum Surface {
    /// A plane
    Plane(Plane),
}

impl Surface {
    /// Construct a `Surface` that represents the x-y plane
    pub fn x_y_plane() -> Self {
        Self::Plane(Plane {
            origin: Point::origin(),
            u: vector![1., 0., 0.],
            v: vector![0., 1., 0.],
        })
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        match self {
            Self::Plane(plane) => Self::Plane(plane.transform(transform)),
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    ///
    /// Returns an error, if the provided point is not in the surface.
    pub fn point_model_to_surface(&self, point_3d: Point<3>) -> SurfacePoint {
        let point_2d = match self {
            Self::Plane(plane) => plane.point_model_to_surface(point_3d),
        };

        SurfacePoint {
            value: point_2d,
            from: point_3d,
        }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: &Point<2>) -> Point<3> {
        match self {
            Self::Plane(plane) => plane.point_surface_to_model(point),
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: &Vector<2>) -> Vector<3> {
        match self {
            Self::Plane(plane) => plane.vector_surface_to_model(vector),
        }
    }
}
