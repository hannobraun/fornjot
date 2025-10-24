use std::collections::VecDeque;

use fj_math::Point;

use crate::{
    approx::point::ApproxPoint,
    geometry::{CurveGeometry, Increment},
};

pub struct CurveApprox {
    increment: Increment,
    points: VecDeque<Point<1>>,
}

impl CurveApprox {
    pub fn start(increment: Increment) -> Self {
        Self {
            increment,
            points: VecDeque::new(),
        }
    }

    pub fn expand_to_include(&mut self, point: Point<1>) -> bool {
        let mut expanded_approximation = false;

        loop {
            let Some(front) = self.points.front().copied() else {
                self.points
                    .push_front(self.increment.snap_to_multiple(point));
                continue;
            };
            let Some(back) = self.points.back().copied() else {
                self.points
                    .push_back(self.increment.snap_to_multiple(point));
                continue;
            };

            if point < front {
                self.points.push_front(front - self.increment.inner);
                expanded_approximation = true;
                continue;
            }
            if point > back {
                self.points.push_back(back + self.increment.inner);
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
