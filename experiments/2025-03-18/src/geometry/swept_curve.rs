use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};
use itertools::Itertools;

use crate::geometry::{SurfaceGeometry, surface::SurfaceApprox};

use super::{AnchoredCurve, Line, curve::FloatingCurve};

#[derive(Debug)]
pub struct SweptCurve {
    pub u: AnchoredCurve,
    pub v: FloatingCurve,
}

impl SweptCurve {
    pub fn plane_from_coord_system(
        origin: impl Into<Point<3>>,
        axes: [impl Into<Vector<3>>; 2],
    ) -> Self {
        let origin = origin.into();
        let [u, v] = axes.map(Into::into).map(|direction| Line { direction });

        Self {
            u: AnchoredCurve::from_origin_and_curve(origin, u),
            v: FloatingCurve::new(v),
        }
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.u.point_from_local([u]) + self.v.vector_from_local_point([v])
    }

    pub fn flip(&self) -> Self {
        Self {
            u: self.u.clone(),
            v: self.v.flip(),
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            u: self.u.translate(offset),
            v: self.v.clone(),
        }
    }
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
    ) -> SurfaceApprox {
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
                    .curvature
                    .iter()
                    .copied()
                    .map(|point_u| Point::from([point_u.t, min_v])),
            )
            .chain(
                approx_u
                    .curvature
                    .iter()
                    .copied()
                    .map(|point_u| Point::from([point_u.t, max_v])),
            )
            .chain(
                approx_v
                    .curvature
                    .iter()
                    .copied()
                    .map(|point_v| Point::from([min_u, point_v.t])),
            )
            .chain(
                approx_v
                    .curvature
                    .iter()
                    .copied()
                    .map(|point_v| Point::from([max_u, point_v.t])),
            )
            .collect()
        };

        let curvature = approx_u
            .curvature
            .into_iter()
            .cartesian_product(approx_v.curvature)
            .map(|(point_u, point_v)| Point::from([point_u.t, point_v.t]))
            .collect();

        SurfaceApprox {
            curvature,
            boundary,
        }
    }
}
