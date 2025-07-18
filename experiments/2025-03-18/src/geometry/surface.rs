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

    fn approximate(&self, boundary: &Aabb<2>) -> SurfaceApproximation {
        // This doesn't take the curvature of the surface into account, thus
        // producing incorrect results unless the surface is flat.
        let boundary = {
            let [[min_u, min_v], [max_u, max_v]] = [boundary.min, boundary.max]
                .map(|point| point.coords.components);

            [
                [min_u, min_v],
                [min_u, max_v],
                [max_u, min_v],
                [max_u, max_v],
            ]
            .map(Point::from)
            .into_iter()
            .collect()
        };

        SurfaceApproximation {
            curvature: vec![],
            boundary,
        }
    }
}

pub struct SurfaceApproximation {
    /// # The points that approximate the curvature of the surface
    pub curvature: Vec<Point<2>>,

    /// # The points that approximate the boundary of the approximation
    pub boundary: Vec<Point<2>>,
}
