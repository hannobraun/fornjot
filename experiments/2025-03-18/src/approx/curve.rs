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
    pub fn new(
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

    /// # Expand the curve approximation to include the provided point
    ///
    /// Only add one new point to the approximation and return that. The caller
    /// may need to call this method repeatedly, until it returns `None`, to
    /// make sure that the provided point has been included in the
    /// approximation.
    #[must_use]
    pub fn expand_to_include(&mut self, point: Point<1>) -> Option<Point<1>> {
        let increment =
            self.geometry
                .increment_at(point, self.tolerance, self.size_hint);

        let Some(front) = self.points.front().copied() else {
            let new_point = increment.snap_to_multiple(point);
            self.points.push_front(new_point);
            return Some(new_point);
        };
        let Some(back) = self.points.back().copied() else {
            let new_point = increment.snap_to_multiple(point);
            self.points.push_back(new_point);
            return Some(new_point);
        };

        if point < front {
            let new_point = front - increment.inner;
            self.points.push_front(new_point);
            return Some(new_point);
        }
        if point > back {
            let new_point = back + increment.inner;
            self.points.push_back(new_point);
            return Some(new_point);
        }

        None
    }

    pub fn into_points(self) -> Vec<Point<1>> {
        self.points.into()
    }
}

pub struct PartialCurveAnchoredApprox {
    pub points: Vec<ApproxPoint<1>>,
}

#[derive(Debug)]
pub struct PartialCurveFloatingApprox {
    pub points: Vec<Point<1>>,
}

impl PartialCurveFloatingApprox {
    pub fn into_anchored(
        self,
        origin: Point<3>,
        curve: &dyn CurveGeometry,
    ) -> PartialCurveAnchoredApprox {
        let points = self
            .points
            .into_iter()
            .map(|point| ApproxPoint::from_curve_point(origin, point, curve))
            .collect();

        PartialCurveAnchoredApprox { points }
    }
}
