use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point};

pub trait SurfaceGeometry: fmt::Debug {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox;
}

pub struct SurfaceApprox {
    pub points: Vec<Point<2>>,
}
