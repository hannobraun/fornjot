use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};
use itertools::Itertools;

use crate::{
    approx::curve::CurveApprox,
    geometry::{SurfaceGeometry, surface::SurfaceApprox},
};

use super::{CurveAnchored, CurveFloating, Line};

/// # A curve that is swept along the path of another curve, forming a surface
///
/// ## Degenerate Case
///
/// There is one known degenerate case (and possibly many more that are hidden)
/// that can happen when constructing a `SweptCurve`.
///
/// If the curves that define the `u` and `v` axis lie in the same surface, and
/// at least one of them is curved by more than 180°, then a degenerate area is
/// formed where the curved curve folds into itself. There, multiple distinct
/// surface points can map to the same global point, creating degenerate
/// triangles during meshing.
///
/// Defining such a curve is valid though, and there are legitimate use cases
/// for it. For example, if a face is swept along a circular curve, then all of
/// its half-edges are swept along that curve, and one of them might happen to
/// be in the same plane as the circle.
///
/// This is generally fine, as long as you are careful when creating a mesh for
/// that surface, by restricting the boundary of the mesh so that no ambiguous
/// surface points are created. In the case of a circle, for example, this would
/// mean that you restrict the boundary of the meshing along the circle's axis
/// to an area of at most 2π (like -π..π or 0..2π).
#[derive(Debug)]
pub struct SweptCurve {
    /// # The curve that is being swept
    ///
    /// Defines the u-axis of the resulting surface. This is an anchored curve,
    /// so it also defines the surface's origin.
    pub u: CurveAnchored,

    /// # The curve along which the `u` curve is being swept
    ///
    /// Defines the v-axis of the resulting surface.
    pub v: CurveFloating,
}

impl SweptCurve {
    pub fn plane_from_coord_system(
        origin: impl Into<Point<3>>,
        axes: [impl Into<Vector<3>>; 2],
    ) -> Self {
        let origin = origin.into();
        let [u, v] = axes.map(Into::into).map(|direction| Line { direction });

        Self {
            u: CurveAnchored::from_origin_and_curve(origin, u),
            v: CurveFloating::new(v),
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
        let [size_hint_u, size_hint_v] =
            (boundary.max - boundary.min).components;

        let [mut approx_u, mut approx_v] = [
            (self.u.floating.geometry.as_ref(), size_hint_u),
            (self.v.geometry.as_ref(), size_hint_v),
        ]
        .map(|(curve, size_hint)| {
            CurveApprox::new(curve, tolerance, size_hint)
        });

        let [[min_u, min_v], [max_u, max_v]] = [boundary.min, boundary.max]
            .map(|point| point.coords.components.map(|s| Point::from([s])));

        loop {
            if approx_u.expand_to_include(min_u).is_some() {
                continue;
            }
            if approx_u.expand_to_include(max_u).is_some() {
                continue;
            }

            break;
        }
        loop {
            if approx_v.expand_to_include(min_v).is_some() {
                continue;
            }
            if approx_v.expand_to_include(max_v).is_some() {
                continue;
            }

            break;
        }

        let approx_u = approx_u.into_points();
        let approx_v = approx_v.into_points();

        let points = approx_u
            .into_iter()
            .cartesian_product(approx_v)
            .map(|(point_u, point_v)| Point::from([point_u.t, point_v.t]))
            .collect();

        SurfaceApprox { points }
    }
}
