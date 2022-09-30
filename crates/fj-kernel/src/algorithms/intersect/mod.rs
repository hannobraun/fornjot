//! Intersection algorithms

pub mod face_point;
pub mod ray_edge;
pub mod ray_face;
pub mod ray_segment;

mod curve_edge;
mod curve_face;
mod face_face;
mod line_segment;
mod surface_surface;

use fj_math::{Point, Vector};

pub use self::{
    curve_edge::CurveEdgeIntersection,
    curve_face::{CurveFaceIntersection, CurveFaceIntersectionInterval},
    face_face::FaceFaceIntersection,
    line_segment::LineSegmentIntersection,
    surface_surface::SurfaceSurfaceIntersection,
};

/// Compute the intersection between a tuple of objects
///
/// # Implementation Note
///
/// This trait is newer than most of the intersection algorithms in this module.
/// Most of them don't support it yet.
pub trait Intersect {
    /// The type that describes the intersection between the objects in `Self`
    type Intersection;

    /// Compute the intersection between a tuple of objects
    fn intersect(self) -> Option<Self::Intersection>;
}

/// A horizontal ray that goes to the right
///
/// For in-kernel use, we don't need anything more flexible, and being exactly
/// horizontal simplifies some calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HorizontalRayToTheRight<const D: usize> {
    /// The point where the ray originates
    pub origin: Point<D>,
}

impl<const D: usize> HorizontalRayToTheRight<D> {
    /// Access the direction of this ray
    pub fn direction(&self) -> Vector<D> {
        let mut components = [0.; D];
        components[0] = 1.;
        components.into()
    }
}

impl<P, const D: usize> From<P> for HorizontalRayToTheRight<D>
where
    P: Into<Point<D>>,
{
    fn from(point: P) -> Self {
        Self {
            origin: point.into(),
        }
    }
}
