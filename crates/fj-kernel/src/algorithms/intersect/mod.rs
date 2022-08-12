//! Intersection algorithms

pub mod face_point;

mod curve_edge;
mod curve_face;
mod face_face;
mod line_segment;
mod surface_surface;

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
