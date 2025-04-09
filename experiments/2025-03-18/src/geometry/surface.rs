use fj_math::{Point, Vector};
use geo::Polygon;

use crate::geometry::SweptCurve;

pub trait SurfaceGeometry {
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
        vec![]
    }
}
