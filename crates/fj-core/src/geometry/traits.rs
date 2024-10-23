//! # Traits that abstract over curve or surface geometry
//!
//! Fornjot's geometry is built on the concept of a uniform representation:
//! Polylines to represent curves and triangle meshes to represent surfaces. The
//! traits in this module provide the interface between this uniform
//! representation and specific geometry code.
//!
//! ## Determinism
//!
//! Uniform representation must be deterministic. That means a given geometric
//! curve or surface, at a given tolerance, must generate the same uniform
//! representation, regardless of where it is queried, and in which order.
//!
//! ## Implementation Note
//!
//! As of this writing, the transition from the previous, more limited, geometry
//! system to the new one based on uniform representation is still ongoing. As a
//! result of that, this module might still be incomplete.

use std::ops::Deref;

use fj_math::{Aabb, LineSegment, Point, Scalar, Triangle};

use super::{CurveBoundary, Geometry, Path, Tolerance};

/// # Generate polylines, the uniform representation of curve geometry
///
/// This trait provides a generic and uniform interface to curve geometry. It is
/// implemented by types that represent specific kinds of curve geometry.
///
/// It is generic over the dimensionality of the generated polyline. Typically,
/// two variants should be implemented per curve geometry type:
///
/// - `GenPolyline<2>` for surface-local geometry.
/// - `GenPolyline<3>` for global 3D geometry.
pub trait GenPolyline<const D: usize> {
    /// # Access the origin of the curve
    fn origin(&self) -> Point<D>;

    /// # Compute a line segment to approximate the curve at this point
    ///
    /// ## Degenerate Case
    ///
    /// If the curve requires no approximation (meaning it is a line), then per
    /// convention, a degenerate line segment is returned, that collapses to the
    /// provided point.
    fn line_segment_at(
        &self,
        point_curve: Point<1>,
        tolerance: Tolerance,
    ) -> LineSegment<D>;

    /// # Generate a polyline within the provided boundary
    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        tolerance: Tolerance,
    ) -> Vec<Point<1>>;
}

// This implementation is temporary, to ease the transition towards a curve
// geometry trait. Eventually, `CurveGeom2` is expected to replace `Path`.
impl<const D: usize> GenPolyline<D> for Path<D> {
    fn origin(&self) -> Point<D> {
        match self {
            Self::Circle(circle) => circle.origin(),
            Self::Line(line) => line.origin(),
        }
    }

    fn line_segment_at(
        &self,
        point_curve: Point<1>,
        tolerance: Tolerance,
    ) -> LineSegment<D> {
        match self {
            Self::Circle(circle) => {
                circle.line_segment_at(point_curve, tolerance)
            }
            Self::Line(line) => line.line_segment_at(point_curve, tolerance),
        }
    }

    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        match self {
            Self::Circle(circle) => {
                circle.generate_polyline(boundary, tolerance)
            }
            Self::Line(line) => line.generate_polyline(boundary, tolerance),
        }
    }
}

/// # Generate triangle meshes, the uniform representation of surface geometry
pub trait GenTriMesh {
    /// # Access the origin of the surface
    fn origin(&self, geometry: &Geometry) -> Point<3>;

    /// # Return the triangle at the provided point on the surface
    ///
    /// Select a triangle of the surface's triangle mesh representation, the one
    /// at the provided surface point. Return that triangle, as well as the
    /// barycentric coordinates of the provided point on the triangle.
    ///
    /// ## Triangle Size and Validity
    ///
    /// If a surface is curved along both axes, the triangle's size is chosen
    /// such, that it approximates the surface, with the maximum allowed
    /// deviation of the actual surface defined by the provided tolerance
    /// argument.
    ///
    /// Otherwise, the size of the returned triangle is at least partially
    /// arbitrary. Take the extreme case of a plane: Since it is not curved at
    /// all, the returned triangle can be arbitrarily large.
    ///
    /// However, since surfaces are infinite, and we can't represent infinite
    /// triangles, there is no sensible upper bound for the size. Instead, to
    /// prevent an arbitrary choice for the size of triangles, which would imply
    /// properties of the surface that are not true, and might therefore be
    /// confusing, the triangles returned by this function have a length of zero
    /// along axes that do not require approximation.
    ///
    /// The most extreme case would be a plane, for which the returned triangle
    /// is collapsed to a point. For a cylinder, the triangle would have the
    /// appropriate width to approximate the curved axis given the provided
    /// tolerance, while having zero height.
    ///
    /// ## Implementation Note
    ///
    /// At the time this was written, there was no dedicated type to represent
    /// barycentric coordinates. Nor any other code that used them, I think.
    ///
    /// If this changes, and a special type for barycentric coordinates is
    /// added, it would make sense to return that here.
    fn triangle_at(
        &self,
        point_surface: Point<2>,
        tolerance: Tolerance,
    ) -> (Triangle<3>, [Scalar; 3]);

    /// # Generated a triangle mesh within the provided boundary
    fn generate_tri_mesh(
        &self,
        boundary: Aabb<2>,
        tolerance: Tolerance,
    ) -> Vec<Point<2>>;
}

impl<T> GenTriMesh for T
where
    T: Deref,
    T::Target: GenTriMesh,
{
    fn origin(&self, geometry: &Geometry) -> Point<3> {
        self.deref().origin(geometry)
    }

    fn triangle_at(
        &self,
        point_surface: Point<2>,
        tolerance: Tolerance,
    ) -> (Triangle<3>, [Scalar; 3]) {
        self.deref().triangle_at(point_surface, tolerance)
    }

    fn generate_tri_mesh(
        &self,
        boundary: Aabb<2>,
        tolerance: Tolerance,
    ) -> Vec<Point<2>> {
        self.deref().generate_tri_mesh(boundary, tolerance)
    }
}
