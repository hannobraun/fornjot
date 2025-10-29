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

    #[must_use]
    pub fn expand_to_include(&mut self, point: Point<1>) -> Option<()> {
        let increment =
            self.geometry
                .increment_at(point, self.tolerance, self.size_hint);

        let Some(front) = self.points.front().copied() else {
            let new_point = increment.snap_to_multiple(point);
            self.points.push_front(new_point);
            return Some(());
        };
        let Some(back) = self.points.back().copied() else {
            let new_point = increment.snap_to_multiple(point);
            self.points.push_back(new_point);
            return Some(());
        };

        if point < front {
            let new_point = front - increment.inner;
            self.points.push_front(new_point);
            return Some(());
        }
        if point > back {
            self.points.push_back(back + increment.inner);
            return Some(());
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
