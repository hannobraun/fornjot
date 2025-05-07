use std::fmt;

use fj_math::{Point, Vector};
use geo::Polygon;

use crate::geometry::SweptCurve;

pub trait SurfaceGeometry: fmt::Debug {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn project_point(&self, point: Point<3>) -> Point<2>;
    fn flip(&self) -> Box<dyn SurfaceGeometry>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry>;

    /// # Approximate the surface
    ///
    /// Returns a set of points, in surface coordinates, that approximate the
    /// surface. The points returned must be within the provided boundary. Not
    /// outside of it, and not on it.
    ///
    /// ## Implementation Notes
    ///
    /// This method should take a tolerance parameter, to define how far the
    /// approximation is allowed to deviate from the actual surface. So far,
    /// this has not been necessary.
    ///
    /// ---
    ///
    /// There is an alternative approach to approximation that could also be
    /// viable, which starts with providing the boundary as a 3D polyline
    /// instead of a surface-local polygon.
    ///
    /// The boundary is necessary in the first place, because we need to
    /// approximate finite faces, not infinite surfaces. Thus the boundary
    /// derives from the half-edges that bound a face. And those are
    /// approximated as a 3D polyline.
    ///
    /// If we could use that polyline directly as the boundary, then the generic
    /// triangulation code would not need to project its points into the
    /// surface. And then the `SurfaceGeometry` trait would not need to provide
    /// that operation, thus simplifying it. This simplification would be the
    /// motivation for the alternative approach.
    ///
    /// The surface-specific approximation code could then do the projecting
    /// itself, using its surface-specific knowledge, or use some other means,
    /// if that's more appropriate.
    ///
    /// However, the surface-local boundary polygon is also used by the generic
    /// approximation code to filter the triangles afterwards, since the
    /// triangles created from the approximation points would cover any holes in
    /// the boundary. With this alternative approach, this surface-specific code
    /// would need to do this, thus this method would need to return triangles
    /// directly.
    ///
    /// And that would complicate the surface-specific code, thus possibly
    /// offsetting any simplification achieved by not having to provide a
    /// projection operation.
    ///
    /// It may be worth it to revisit this approach, once all of this
    /// approximation and triangulation business is a bit more fleshed out.
    /// Maybe there are gains to be had, or maybe not.
    fn approximate(&self, boundary: &Polygon) -> Vec<Point<2>>;
}

impl SurfaceGeometry for SweptCurve {
    fn point_from_local(&self, point: Point<2>) -> Point<3> {
        self.point_from_local(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<2> {
        self.project_point(point)
    }

    fn flip(&self) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).flip())
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).translate(offset))
    }

    fn approximate(&self, _: &Polygon) -> Vec<Point<2>> {
        // In a swept curve, the curve sweeps along a straight path. So the
        // surface is only curved along one dimension.
        //
        // As a result, all points that could possibly be needed to approximate
        // the surface, are already on the provided boundary. As per the
        // contract of this method, we must not return those.
        vec![]
    }
}
