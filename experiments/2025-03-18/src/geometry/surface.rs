use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};
use itertools::Itertools;

use crate::geometry::SweptCurve;

pub trait SurfaceGeometry: fmt::Debug {
    fn point_from_local(&self, point: Point<2>) -> Point<3>;
    fn flip(&self) -> Box<dyn SurfaceGeometry>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry>;

    /// # Approximate the surface
    ///
    /// ## Implementation Note
    ///
    /// This method should take a tolerance parameter, to define how far the
    /// approximation is allowed to deviate from the actual surface. So far,
    /// this has not been necessary.
    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApproximation;
}

impl SurfaceGeometry for SweptCurve {
    fn point_from_local(&self, point: Point<2>) -> Point<3> {
        self.point_from_local(point)
    }

    fn flip(&self) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).flip())
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn SurfaceGeometry> {
        Box::new((*self).translate(offset))
    }

    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApproximation {
        let [[min_u, min_v], [max_u, max_v]] =
            [boundary.min, boundary.max].map(|point| point.coords.components);

        let approx_u = self.u.approximate([[min_u], [max_u]], tolerance);
        let approx_v = self.v.approximate([[min_v], [max_v]], tolerance);

        let boundary = {
            [
                [min_u, min_v],
                [min_u, max_v],
                [max_u, min_v],
                [max_u, max_v],
            ]
            .map(Point::from)
            .into_iter()
            .chain(
                approx_u
                    .iter()
                    .copied()
                    .map(|point_u| Point::from([point_u.t, min_v])),
            )
            .chain(
                approx_u
                    .iter()
                    .copied()
                    .map(|point_u| Point::from([point_u.t, max_v])),
            )
            .chain(
                approx_v
                    .iter()
                    .copied()
                    .map(|point_v| Point::from([min_u, point_v.t])),
            )
            .chain(
                approx_v
                    .iter()
                    .copied()
                    .map(|point_v| Point::from([max_u, point_v.t])),
            )
            .collect()
        };

        let curvature = approx_u
            .into_iter()
            .cartesian_product(approx_v)
            .map(|(point_u, point_v)| Point::from([point_u.t, point_v.t]))
            .collect();

        SurfaceApproximation {
            curvature,
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
