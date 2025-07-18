use std::fmt;

use fj_math::{Aabb, Point, Vector};

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
    /// ## Implementation Note
    ///
    /// This method should take a tolerance parameter, to define how far the
    /// approximation is allowed to deviate from the actual surface. So far,
    /// this has not been necessary.
    fn approximate(&self, boundary: &Aabb<2>) -> SurfaceApproximation;
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

    fn approximate(&self, _: &Aabb<2>) -> SurfaceApproximation {
        // In a swept curve, the curve sweeps along a straight path. So the
        // surface is only curved along one dimension.
        //
        // As a result, all points that could possibly be needed to approximate
        // the surface, are already on the provided boundary. As per the
        // contract of this method, we must not return those.
        SurfaceApproximation { curvature: vec![] }
    }
}

pub struct SurfaceApproximation {
    /// # The points that approximate the curvature of the surface
    pub curvature: Vec<Point<2>>,
}
