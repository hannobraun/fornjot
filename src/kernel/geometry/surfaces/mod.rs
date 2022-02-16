pub mod swept;

pub use self::swept::Swept;

use nalgebra::vector;

use crate::kernel::math::{Point, Transform, Vector};

use super::{points::SurfacePoint, Curve, Line};

/// A two-dimensional shape
#[derive(Clone, Debug, PartialEq)]
pub enum Surface {
    /// A swept curve
    Swept(Swept),
}

impl Surface {
    /// Construct a `Surface` that represents the x-y plane
    pub fn x_y_plane() -> Self {
        Self::Swept(Swept {
            curve: Curve::Line(Line {
                origin: Point::origin(),
                direction: vector![1., 0., 0.],
            }),
            path: vector![0., 1., 0.],
        })
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Swept(surface) => Self::Swept(surface.transform(transform)),
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point_3d: Point<3>) -> SurfacePoint {
        let point_2d = match self {
            Self::Swept(surface) => surface.point_model_to_surface(&point_3d),
        };

        SurfacePoint {
            value: point_2d,
            from: point_3d,
        }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: &Point<2>) -> Point<3> {
        match self {
            Self::Swept(surface) => surface.point_surface_to_model(point),
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: &Vector<2>) -> Vector<3> {
        match self {
            Self::Swept(surface) => surface.vector_surface_to_model(vector),
        }
    }
}
