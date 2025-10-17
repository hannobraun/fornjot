use fj_math::Point;

use crate::{approx::point::ApproxPoint, geometry::CurveGeometry};

pub struct CurveApproxAnchored {
    /// # The points that approximate the curvature of the curve
    ///
    /// This does not include the boundary of the approximation.
    pub curvature: Vec<ApproxPoint<1>>,
}

#[derive(Debug)]
pub struct CurveApproxFloating {
    /// # The points that approximate the curvature of the curve
    ///
    /// This does not include the boundary of the approximation.
    pub curvature: Vec<Point<1>>,
}

impl CurveApproxFloating {
    pub fn into_anchored(
        self,
        origin: Point<3>,
        curve: &dyn CurveGeometry,
    ) -> CurveApproxAnchored {
        let curvature = self
            .curvature
            .into_iter()
            .map(|point| ApproxPoint::from_curve_point(origin, point, curve))
            .collect();

        CurveApproxAnchored { curvature }
    }
}
