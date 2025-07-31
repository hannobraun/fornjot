use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};

pub trait SurfaceGeometry: fmt::Debug {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn flip(&self) -> Box<dyn SurfaceGeometry>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry>;
    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox;
}

pub struct SurfaceApprox {
    /// # The points that approximate the curvature of the surface
    ///
    /// This does not include the points that approximate the boundary of the
    /// approximation.
    pub curvature: Vec<Point<2>>,

    /// # The points that approximate the boundary of the approximation
    pub boundary: Vec<Point<2>>,
}
