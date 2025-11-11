use std::{fmt, rc::Rc};

use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};

pub trait SurfaceGeometry: fmt::Debug {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn flip(&self) -> Rc<dyn SurfaceGeometry>;
    fn translate(&self, offset: Vector<3>) -> Rc<dyn SurfaceGeometry>;
    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox;
}

pub struct SurfaceApprox {
    pub points: Vec<Point<2>>,
}
