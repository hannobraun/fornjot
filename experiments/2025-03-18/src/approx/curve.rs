use std::collections::VecDeque;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar};

use crate::{approx::point::ApproxPoint, geometry::CurveGeometry};

pub struct CurveApprox<'r> {
    geometry: &'r dyn CurveGeometry,
    tolerance: Tolerance,
    size_hint: Scalar,
    points: VecDeque<Point<1>>,
}

impl<'r> CurveApprox<'r> {
    pub fn start(
        geometry: &'r dyn CurveGeometry,
        tolerance: Tolerance,
        size_hint: Scalar,
    ) -> Self {
        Self {
            geometry,
            tolerance,
            size_hint,
            points: VecDeque::new(),
        }
    }

    pub fn expand_to_include(&mut self, point: Point<1>) -> bool {
        let increment = self.geometry.increment(self.tolerance, self.size_hint);

        let mut expanded_approximation = false;

        loop {
            let Some(front) = self.points.front().copied() else {
                self.points.push_front(increment.snap_to_multiple(point));
                continue;
            };
            let Some(back) = self.points.back().copied() else {
                self.points.push_back(increment.snap_to_multiple(point));
                continue;
            };

            if point < front {
                self.points.push_front(front - increment.inner);
                expanded_approximation = true;
                continue;
            }
            if point > back {
                self.points.push_back(back + increment.inner);
                expanded_approximation = true;
                continue;
            }

            break;
        }

        expanded_approximation
    }

    pub fn into_points(self) -> Vec<Point<1>> {
        self.points.into()
    }
}

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
